// platform/windows/win_context.rs
// Windows context menu extraction and execution with background pre-caching.

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WinContextItem {
    pub id: String,
    pub title: String,
    pub command: String,
    pub icon: Option<String>,
    pub applies_to: String, // "file", "directory", "background", "all"
}

static WIN_CONTEXT_CACHE: OnceLock<Vec<WinContextItem>> = OnceLock::new();

/// Pre-cache Windows context menu items in a background thread at startup.
pub fn init_cache_background() {
    #[cfg(target_os = "windows")]
    std::thread::spawn(|| {
        let items = read_all_win_context_items();
        let count = items.len();
        let _ = WIN_CONTEXT_CACHE.set(items);
        tracing::info!("Windows context menu cache initialized with {} items", count);
    });
}

#[cfg(target_os = "windows")]
pub fn read_all_win_context_items() -> Vec<WinContextItem> {
    use winreg::enums::*;
    use winreg::RegKey;

    let mut items = Vec::new();
    let mut seen_commands = std::collections::HashSet::new();

    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
    let hkcu_classes = RegKey::predef(HKEY_CURRENT_USER).open_subkey(r"Software\Classes").ok();

    // 1. Directory Background (applies_to = "background")
    read_verbs_from_key(&hkcr, r"Directory\Background\shell", "background", &mut items, &mut seen_commands);
    if let Some(ref hkcu) = hkcu_classes {
        read_verbs_from_key(hkcu, r"Directory\Background\shell", "background", &mut items, &mut seen_commands);
    }

    // 2. Directory / Folder (applies_to = "directory")
    read_verbs_from_key(&hkcr, r"Directory\shell", "directory", &mut items, &mut seen_commands);
    read_verbs_from_key(&hkcr, r"Folder\shell", "directory", &mut items, &mut seen_commands);
    if let Some(ref hkcu) = hkcu_classes {
        read_verbs_from_key(hkcu, r"Directory\shell", "directory", &mut items, &mut seen_commands);
        read_verbs_from_key(hkcu, r"Folder\shell", "directory", &mut items, &mut seen_commands);
    }

    // 3. Files (applies_to = "file")
    read_verbs_from_key(&hkcr, r"*\shell", "file", &mut items, &mut seen_commands);
    read_verbs_from_key(&hkcr, r"AllFilesystemObjects\shell", "all", &mut items, &mut seen_commands);
    if let Some(ref hkcu) = hkcu_classes {
        read_verbs_from_key(hkcu, r"*\shell", "file", &mut items, &mut seen_commands);
        read_verbs_from_key(hkcu, r"AllFilesystemObjects\shell", "all", &mut items, &mut seen_commands);
    }

    items
}

#[cfg(target_os = "windows")]
fn read_verbs_from_key(
    root: &winreg::RegKey,
    subkey_path: &str,
    applies_to: &str,
    items: &mut Vec<WinContextItem>,
    seen_commands: &mut std::collections::HashSet<String>,
) {
    let Ok(shell_key) = root.open_subkey(subkey_path) else {
        return;
    };

    for subkey_name in shell_key.enum_keys().filter_map(|r| r.ok()) {
        let Ok(item_key) = shell_key.open_subkey(&subkey_name) else {
            continue;
        };

        // Must have a command subkey with a default value
        let Ok(cmd_key) = item_key.open_subkey("command") else {
            continue;
        };

        let Ok(raw_command): Result<String, _> = cmd_key.get_value("") else {
            continue;
        };

        let trimmed_cmd = raw_command.trim().to_string();
        if trimmed_cmd.is_empty() {
            continue;
        }

        // Title: check MUIVerb, then (default), then subkey_name
        let title: String = item_key.get_value("MUIVerb")
            .unwrap_or_else(|_| item_key.get_value("").unwrap_or_default());
        
        let title = title.trim();
        let display_title = if title.is_empty() {
            clean_verb_name(&subkey_name)
        } else {
            title.replace('&', "").trim().to_string()
        };

        if display_title.is_empty() {
            continue;
        }

        // Filter out Windows internal verb handlers that aren't useful in Explorer context
        let lower_verb = subkey_name.to_lowercase();
        if lower_verb == "pintohomefile" 
            || lower_verb == "removeproperties" 
            || lower_verb == "updateencryptionsettings"
            || lower_verb == "updateencryptionsettingswork" 
            || lower_verb == "modernproperties"
            || lower_verb == "offlinefileslaunchsynccenter"
            || lower_verb == "launchworkfolderscontrol" {
            continue;
        }

        // Icon (optional)
        let icon: Option<String> = item_key.get_value("Icon").ok();

        // Dedup key: (applies_to, display_title, trimmed_cmd)
        let dedup_key = format!("{}:{}:{}", applies_to, display_title, trimmed_cmd);
        if seen_commands.contains(&dedup_key) {
            continue;
        }
        seen_commands.insert(dedup_key);

        items.push(WinContextItem {
            id: format!("{}_{}", applies_to, subkey_name),
            title: display_title,
            command: trimmed_cmd,
            icon,
            applies_to: applies_to.to_string(),
        });
    }
}

#[cfg(target_os = "windows")]
fn clean_verb_name(name: &str) -> String {
    match name.to_lowercase().as_str() {
        "vscode" => "Visual Studio Code".to_string(),
        "powershell" => "PowerShell".to_string(),
        "cmd" => "命令提示符 (CMD)".to_string(),
        "git_shell" => "Git Bash".to_string(),
        "git_gui" => "Git GUI".to_string(),
        "gitkraken" => "GitKraken".to_string(),
        _ => name.replace('_', " ").trim().to_string(),
    }
}

pub fn get_context_menu_impl() -> Result<Vec<WinContextItem>, String> {
    #[cfg(target_os = "windows")]
    {
        if let Some(items) = WIN_CONTEXT_CACHE.get() {
            return Ok(items.clone());
        }
        let items = read_all_win_context_items();
        let _ = WIN_CONTEXT_CACHE.set(items.clone());
        Ok(items)
    }
    #[cfg(not(target_os = "windows"))]
    {
        Ok(Vec::new())
    }
}

pub fn execute_context_item_impl(command: String, target_path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let target = target_path.replace('/', "\\");
        let mut final_cmd = command;
        
        let mut replaced = false;
        for token in &["\"%1\"", "\"%V\"", "\"%L\"", "\"%W\"", "%1", "%V", "%L", "%W"] {
            if final_cmd.contains(token) {
                final_cmd = final_cmd.replace(token, &format!("\"{}\"", target));
                replaced = true;
            }
        }

        if !replaced && !target.is_empty() {
            final_cmd = format!("{} \"{}\"", final_cmd, target);
        }

        tracing::info!("Executing Windows context command: {}", final_cmd);
        
        let status = std::process::Command::new("cmd")
            .args(["/c", &format!("start \"\" {}", final_cmd)])
            .spawn();

        match status {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to execute command: {}", e)),
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = (command, target_path);
        Err("Windows context menu is only supported on Windows".into())
    }
}
