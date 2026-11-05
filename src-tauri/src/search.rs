use crate::core::error::AppResult;
use crate::core::state::AppState;
use crate::core::types::{FileEntry, SearchProgress, SEARCH_BATCH_SIZE, SEARCH_MAX_RESULTS};
use crate::engine::{EverythingEngine, SearchOptions};
use std::path::{Path, PathBuf};
use std::sync::atomic::Ordering;
use std::sync::OnceLock;
use tauri::{AppHandle, Emitter, State};

static EVERYTHING_ENGINE: OnceLock<EverythingEngine> = OnceLock::new();

/// Return or initialize the global Everything core engine for files-explorer
pub fn get_everything_engine() -> &'static EverythingEngine {
    EVERYTHING_ENGINE.get_or_init(|| {
        let engine = EverythingEngine::new();
        // Index all local drives in the background
        let engine_clone = engine.clone();
        std::thread::spawn(move || {
            let _ = engine_clone.index_all_drives();
        });
        engine
    })
}

pub fn wildcard_match(pattern: &str, filename: &str) -> bool {
    crate::engine::query::wildcard_match(pattern, filename, false)
}

/// Parse a size filter like ">10MB" or "<1GB" into (operator, bytes)
pub fn parse_size_filter(s: &str) -> Option<(char, u64)> {
    let s = s.trim();
    let op = s.chars().next()?;
    if op != '>' && op != '<' {
        return None;
    }
    let rest = s[1..].trim();
    let num_end = rest
        .find(|c: char| !c.is_ascii_digit() && c != '.')
        .unwrap_or(rest.len());
    if num_end == 0 {
        return None;
    }
    let num: f64 = rest[..num_end].parse().ok()?;
    let unit = rest[num_end..].trim().to_lowercase();
    let multiplier = match unit.as_str() {
        "b" | "" => 1.0,
        "k" | "kb" => 1024.0,
        "m" | "mb" => 1024.0 * 1024.0,
        "g" | "gb" => 1024.0 * 1024.0 * 1024.0,
        "t" | "tb" => 1024.0 * 1024.0 * 1024.0 * 1024.0,
        _ => return None,
    };
    Some((op, (num * multiplier) as u64))
}

/// Check whether a single condition matches a file
pub fn condition_matches_file(condition: &str, name: &str, size: u64) -> bool {
    let cond = condition.trim();
    if cond.is_empty() {
        return false;
    }
    if let Some((op, threshold)) = parse_size_filter(cond) {
        return match op {
            '>' => size > threshold,
            '<' => size < threshold,
            _ => false,
        };
    }
    if cond.contains('*') || cond.contains('?') {
        return wildcard_match(cond, name);
    }
    name.to_lowercase().contains(&cond.to_lowercase())
}

/// Run Everything search in a dedicated thread so it never blocks the UI
pub fn search_files(
    app: AppHandle,
    state: State<AppState>,
    directory: String,
    query: String,
) -> AppResult<()> {
    // Reset cancel flag
    state.search_cancel.store(false, Ordering::SeqCst);
    let cancel = state.search_cancel.clone();

    let q = query.trim().to_string();
    if q.is_empty() {
        let _ = app.emit(
            "search-progress",
            SearchProgress {
                files: vec![],
                total: 0,
                done: true,
                truncated: false,
            },
        );
        return Ok(());
    }

    let dir = directory.clone();
    let app_clone = app.clone();
    let engine = get_everything_engine();

    std::thread::spawn(move || {
        let dir_path = Path::new(&dir);
        let has_dir = !dir.is_empty() && dir != "/" && dir_path.exists();
        let (filter_dir, search_query) = if has_dir {
            // If the folder is specific and not a root drive, index it quickly to catch any fresh files
            if dir_path.parent().is_some() || engine.get_indexed_count() < 1000 {
                engine.index_path(dir_path);
            }
            let mut norm = dir.replace('/', "\\");
            if !norm.ends_with('\\') {
                norm.push('\\');
            }
            let norm_lower = norm.to_lowercase();
            let clean_dir_token = norm.trim_end_matches('\\');
            let q_with_path = if !q.to_lowercase().contains("path:") {
                format!("{} path:\"{}\"", q, clean_dir_token)
            } else {
                q.clone()
            };
            (Some(norm_lower), q_with_path)
        } else {
            (None, q.clone())
        };

        let mut options = SearchOptions::default();
        if filter_dir.is_none() {
            options.max_results = Some(SEARCH_MAX_RESULTS as usize);
        }

        let (entries, _) = engine.search(&search_query, &options);

        let mut batch: Vec<FileEntry> = Vec::new();
        let mut total: u64 = 0;
        let mut truncated = false;

        for entry in entries {
            if cancel.load(Ordering::SeqCst) {
                let files = std::mem::take(&mut batch);
                let _ = app_clone.emit(
                    "search-progress",
                    SearchProgress {
                        files,
                        total,
                        done: true,
                        truncated: false,
                    },
                );
                return;
            }

            if let Some(ref d) = filter_dir {
                let entry_full_lower = entry.full_path.replace('/', "\\").to_lowercase();
                let d_no_slash = d.trim_end_matches('\\');
                if !entry_full_lower.starts_with(d) && entry_full_lower != d_no_slash {
                    continue;
                }
            }

            if total >= SEARCH_MAX_RESULTS {
                truncated = true;
                break;
            }

            total += 1;
            let full_pb = PathBuf::from(&entry.full_path);
            batch.push(FileEntry {
                name: entry.name,
                path: crate::core::fs_helper::path_for_frontend(&full_pb),
                is_dir: entry.is_dir,
                size: entry.size,
                modified: entry.modified,
                created: entry.created,
                extension: entry.extension,
            });

            if batch.len() >= SEARCH_BATCH_SIZE {
                let files = std::mem::take(&mut batch);
                let payload = SearchProgress {
                    files,
                    total,
                    done: false,
                    truncated: false,
                };
                if app_clone.emit("search-progress", payload).is_err() {
                    break;
                }
            }
        }

        let files = std::mem::take(&mut batch);
        let _ = app_clone.emit(
            "search-progress",
            SearchProgress {
                files,
                total,
                done: true,
                truncated,
            },
        );
    });

    Ok(())
}

pub fn cancel_search(state: State<AppState>) -> AppResult<()> {
    state.search_cancel.store(true, Ordering::SeqCst);
    get_everything_engine().cancel_search();
    Ok(())
}
