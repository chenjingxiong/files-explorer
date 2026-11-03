// platform/windows/win_context.rs
// Windows context menu extraction and execution with disk persistence.

use serde::{Deserialize, Serialize};
use std::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WinContextItem {
    pub id: String,
    pub title: String,
    pub command: String,
    pub icon: Option<String>,
    pub applies_to: String, // "file", "directory", "background", "all"
}

static WIN_CONTEXT_CACHE: RwLock<Vec<WinContextItem>> = RwLock::new(Vec::new());

fn get_cache_file_path() -> std::path::PathBuf {
    use crate::platform::PlatformPath;
    let path_impl = crate::platform::windows::path_impl::PathImpl;
    let dir = path_impl.app_data_dir();
    let _ = std::fs::create_dir_all(&dir);
    dir.join("win_context_cache.json")
}

/// Fast startup: strictly load from pre-saved disk cache only.
/// NEVER scans the registry on startup, ensuring startup remains instantaneous (< 1ms).
pub fn load_cache_from_disk_only() {
    #[cfg(target_os = "windows")]
    {
        let cache_file = get_cache_file_path();
        if cache_file.exists() {
            if let Ok(content) = std::fs::read_to_string(&cache_file) {
                if let Ok(items) = serde_json::from_str::<Vec<WinContextItem>>(&content) {
                    let count = items.len();
                    if let Ok(mut lock) = WIN_CONTEXT_CACHE.write() {
                        *lock = items;
                    }
                    tracing::info!("Loaded {} cached Windows context items from disk (fast startup, 0ms registry)", count);
                    return;
                }
            }
        }
        tracing::info!("No Windows context cache file found; skipping registry scan on startup");
    }
}

/// Backwards compatibility alias for tauri_setup
pub fn init_cache_background() {
    load_cache_from_disk_only();
}

#[cfg(target_os = "windows")]
fn resolve_mui_string(s: &str) -> String {
    if !s.starts_with('@') {
        return s.to_string();
    }
    use std::os::windows::ffi::OsStrExt;
    use std::ffi::OsStr;
    let wide: Vec<u16> = OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect();
    let mut out_buf = [0u16; 1024];

    #[link(name = "shlwapi")]
    extern "system" {
        fn SHLoadIndirectString(
            pszSource: *const u16,
            pszOutBuf: *mut u16,
            cchOutBuf: u32,
            ppvReserved: *mut *mut std::ffi::c_void,
        ) -> i32;
    }

    let hr = unsafe {
        SHLoadIndirectString(wide.as_ptr(), out_buf.as_mut_ptr(), out_buf.len() as u32, std::ptr::null_mut())
    };

    if hr == 0 {
        let len = out_buf.iter().position(|&c| c == 0).unwrap_or(out_buf.len());
        let res = String::from_utf16_lossy(&out_buf[..len]);
        let trimmed = res.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }
    s.to_string()
}

#[cfg(target_os = "windows")]
fn fix_mojibake(s: &str) -> String {
    // If string already contains valid Chinese characters, it is definitely not mojibake
    if s.chars().any(|c| c >= '\u{4e00}' && c <= '\u{9fa5}') {
        return s.to_string();
    }
    if let (cow, _, false) = encoding_rs::GB18030.encode(s) {
        if let Ok(utf8_str) = std::str::from_utf8(&cow) {
            if !utf8_str.is_empty() && utf8_str != s && utf8_str.chars().any(|c| c >= '\u{4e00}' && c <= '\u{9fa5}') {
                return utf8_str.to_string();
            }
        }
    }
    s.to_string()
}

#[cfg(target_os = "windows")]
pub fn read_all_win_context_items() -> Vec<WinContextItem> {
    use winreg::enums::*;
    use winreg::RegKey;

    let mut items = Vec::new();
    let mut seen_commands = std::collections::HashSet::new();

    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
    let hkcu = RegKey::predef(HKEY_CURRENT_USER).open_subkey(r"Software\Classes").ok();
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey(r"Software\Classes").ok();

    let mut roots: Vec<&RegKey> = vec![&hkcr];
    if let Some(ref cu) = hkcu {
        roots.push(cu);
    }
    if let Some(ref lm) = hklm {
        roots.push(lm);
    }

    // 1. Directory Background & Drive Background (applies_to = "background")
    for r in &roots {
        read_verbs_from_key(r, r"Directory\Background\shell", "background", &mut items, &mut seen_commands);
        read_verbs_from_key(r, r"Drive\Background\shell", "background", &mut items, &mut seen_commands);
        read_verbs_from_key(r, r"DesktopBackground\shell", "background", &mut items, &mut seen_commands);
    }

    // 2. Directory / Folder / Drive / SystemFileAssociations (applies_to = "directory")
    for r in &roots {
        read_verbs_from_key(r, r"Directory\shell", "directory", &mut items, &mut seen_commands);
        read_verbs_from_key(r, r"Folder\shell", "directory", &mut items, &mut seen_commands);
        read_verbs_from_key(r, r"Drive\shell", "directory", &mut items, &mut seen_commands);
        read_verbs_from_key(r, r"SystemFileAssociations\directory\shell", "directory", &mut items, &mut seen_commands);
        read_verbs_from_key(r, r"SystemFileAssociations\folder\shell", "directory", &mut items, &mut seen_commands);
    }

    // 3. Files & AllFilesystemObjects & SystemFileAssociations (applies_to = "file" / "all")
    for r in &roots {
        read_verbs_from_key(r, r"*\shell", "file", &mut items, &mut seen_commands);
        read_verbs_from_key(r, r"AllFilesystemObjects\shell", "all", &mut items, &mut seen_commands);
        read_verbs_from_key(r, r"SystemFileAssociations\*\shell", "file", &mut items, &mut seen_commands);
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
        let raw_title: String = item_key.get_value("MUIVerb")
            .unwrap_or_else(|_| item_key.get_value("").unwrap_or_default());
        
        let mut display_title = raw_title.replace('&', "").trim().to_string();
        if display_title.starts_with('@') {
            display_title = resolve_mui_string(&display_title);
        }
        if display_title.starts_with('@') || display_title.is_empty() {
            display_title = clean_verb_name(&subkey_name);
        } else {
            display_title = fix_mojibake(&display_title);
        }

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
        "anycode" => "在 Visual Studio 中打开".to_string(),
        "runas" => "以管理员身份打开 PowerShell".to_string(),
        "open" => "打开".to_string(),
        "explore" => "在资源管理器中浏览".to_string(),
        "opennewwindow" => "在新窗口中打开".to_string(),
        "opennewtab" => "在新标签页中打开".to_string(),
        "find" => "搜索".to_string(),
        _ => name.replace('_', " ").trim().to_string(),
    }
}

/// Retrieve the currently cached context menu items.
/// Strictly returns in-memory items or loads existing disk cache. Never rescans registry.
pub fn get_context_menu_impl() -> Result<Vec<WinContextItem>, String> {
    #[cfg(target_os = "windows")]
    {
        if let Ok(lock) = WIN_CONTEXT_CACHE.read() {
            if !lock.is_empty() {
                return Ok(lock.clone());
            }
        }
        let cache_file = get_cache_file_path();
        if cache_file.exists() {
            if let Ok(content) = std::fs::read_to_string(&cache_file) {
                if let Ok(items) = serde_json::from_str::<Vec<WinContextItem>>(&content) {
                    if let Ok(mut lock) = WIN_CONTEXT_CACHE.write() {
                        *lock = items.clone();
                    }
                    return Ok(items);
                }
            }
        }
        Ok(Vec::new())
    }
    #[cfg(not(target_os = "windows"))]
    {
        Ok(Vec::new())
    }
}

/// Explicitly rescan Windows context menu from registry on user demand,
/// update the disk cache file in app data directory, and update the memory cache.
pub fn refresh_windows_context_menu_impl() -> Result<Vec<WinContextItem>, String> {
    #[cfg(target_os = "windows")]
    {
        tracing::info!("Refreshing Windows context menu from registry upon user request...");
        let items = read_all_win_context_items();
        let count = items.len();
        let cache_file = get_cache_file_path();
        if let Ok(json) = serde_json::to_string_pretty(&items) {
            let _ = std::fs::write(&cache_file, json);
        }
        if let Ok(mut lock) = WIN_CONTEXT_CACHE.write() {
            *lock = items.clone();
        }
        tracing::info!("Windows context menu refreshed: {} items saved to disk at {:?}", count, cache_file);
        Ok(items)
    }
    #[cfg(not(target_os = "windows"))]
    {
        Ok(Vec::new())
    }
}

#[cfg(target_os = "windows")]
fn expand_env_strings(src: &str) -> String {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    let wide_src: Vec<u16> = OsStr::new(src).encode_wide().chain(std::iter::once(0)).collect();
    let mut buf = vec![0u16; 4096];

    extern "system" {
        fn ExpandEnvironmentStringsW(lpSrc: *const u16, lpDst: *mut u16, nSize: u32) -> u32;
    }

    let ret = unsafe { ExpandEnvironmentStringsW(wide_src.as_ptr(), buf.as_mut_ptr(), buf.len() as u32) };
    if ret > 0 && (ret as usize) < buf.len() {
        let len = buf.iter().position(|&c| c == 0).unwrap_or(ret as usize);
        return String::from_utf16_lossy(&buf[..len]);
    }
    src.to_string()
}

#[cfg(target_os = "windows")]
fn spawn_process_wide(cmd_line: &str, work_dir: Option<&str>) -> Result<(), String> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    #[repr(C)]
    #[allow(non_snake_case)]
    struct STARTUPINFOW {
        cb: u32,
        lpReserved: *mut u16,
        lpDesktop: *mut u16,
        lpTitle: *mut u16,
        dwX: u32,
        dwY: u32,
        dwXSize: u32,
        dwYSize: u32,
        dwXCountChars: u32,
        dwYCountChars: u32,
        dwFillAttribute: u32,
        dwFlags: u32,
        wShowWindow: u16,
        cbReserved2: u16,
        lpReserved2: *mut u8,
        hStdInput: *mut std::ffi::c_void,
        hStdOutput: *mut std::ffi::c_void,
        hStdError: *mut std::ffi::c_void,
    }

    #[repr(C)]
    #[allow(non_snake_case)]
    struct PROCESS_INFORMATION {
        hProcess: *mut std::ffi::c_void,
        hThread: *mut std::ffi::c_void,
        dwProcessId: u32,
        dwThreadId: u32,
    }

    extern "system" {
        fn CreateProcessW(
            lpApplicationName: *const u16,
            lpCommandLine: *mut u16,
            lpProcessAttributes: *mut std::ffi::c_void,
            lpThreadAttributes: *mut std::ffi::c_void,
            bInheritHandles: i32,
            dwCreationFlags: u32,
            lpEnvironment: *mut std::ffi::c_void,
            lpCurrentDirectory: *const u16,
            lpStartupInfo: *const STARTUPINFOW,
            lpProcessInformation: *mut PROCESS_INFORMATION,
        ) -> i32;
        fn CloseHandle(hObject: *mut std::ffi::c_void) -> i32;
        fn GetLastError() -> u32;
    }

    let mut cmd_wide: Vec<u16> = OsStr::new(cmd_line).encode_wide().chain(std::iter::once(0)).collect();
    let dir_wide: Option<Vec<u16>> = work_dir.map(|d| OsStr::new(d).encode_wide().chain(std::iter::once(0)).collect());
    let dir_ptr = dir_wide.as_ref().map(|v| v.as_ptr()).unwrap_or(std::ptr::null());

    let mut si: STARTUPINFOW = unsafe { std::mem::zeroed() };
    si.cb = std::mem::size_of::<STARTUPINFOW>() as u32;
    let mut pi: PROCESS_INFORMATION = unsafe { std::mem::zeroed() };

    let success = unsafe {
        CreateProcessW(
            std::ptr::null(),
            cmd_wide.as_mut_ptr(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            0,
            0,
            std::ptr::null_mut(),
            dir_ptr,
            &si,
            &mut pi,
        )
    };

    if success != 0 {
        unsafe {
            CloseHandle(pi.hProcess);
            CloseHandle(pi.hThread);
        }
        Ok(())
    } else {
        let err = unsafe { GetLastError() };
        Err(format!("CreateProcessW failed with error code: {}", err))
    }
}

#[cfg(target_os = "windows")]
fn spawn_shellexecute_wide(cmd_line: &str, work_dir: Option<&str>) -> Result<(), String> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    #[link(name = "shell32")]
    extern "system" {
        fn ShellExecuteW(
            hwnd: *mut std::ffi::c_void,
            lpOperation: *const u16,
            lpFile: *const u16,
            lpParameters: *const u16,
            lpDirectory: *const u16,
            nShowCmd: i32,
        ) -> isize;
    }

    let trimmed = cmd_line.trim();
    let (exe, args) = if trimmed.starts_with('"') {
        if let Some(end_quote) = trimmed[1..].find('"') {
            let exe_part = &trimmed[1..1 + end_quote];
            let rest = trimmed[1 + end_quote + 1..].trim();
            (exe_part, rest)
        } else {
            (trimmed, "")
        }
    } else {
        match trimmed.find(' ') {
            Some(space_idx) => (&trimmed[..space_idx], trimmed[space_idx + 1..].trim()),
            None => (trimmed, ""),
        }
    };

    let exe_wide: Vec<u16> = OsStr::new(exe).encode_wide().chain(std::iter::once(0)).collect();
    let args_wide: Option<Vec<u16>> = if !args.is_empty() {
        Some(OsStr::new(args).encode_wide().chain(std::iter::once(0)).collect())
    } else {
        None
    };
    let dir_wide: Option<Vec<u16>> = work_dir.map(|d| OsStr::new(d).encode_wide().chain(std::iter::once(0)).collect());

    let ret = unsafe {
        ShellExecuteW(
            std::ptr::null_mut(),
            std::ptr::null(),
            exe_wide.as_ptr(),
            args_wide.as_ref().map(|v| v.as_ptr()).unwrap_or(std::ptr::null()),
            dir_wide.as_ref().map(|v| v.as_ptr()).unwrap_or(std::ptr::null()),
            1, // SW_SHOWNORMAL
        )
    };

    if ret > 32 {
        Ok(())
    } else {
        Err(format!("ShellExecuteW failed with error code: {}", ret))
    }
}

pub fn execute_context_item_impl(command_or_id: String, target_path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use std::path::Path;

        let target = target_path.replace('/', "\\");

        // 1. Resolve command_or_id: check if it matches an item id from memory cache
        let mut raw_cmd = command_or_id.clone();
        if let Ok(lock) = WIN_CONTEXT_CACHE.read() {
            if let Some(item) = lock.iter().find(|i| i.id == command_or_id) {
                raw_cmd = item.command.clone();
            }
        }

        // 2. Expand environment variables (e.g. %SystemRoot%, %ProgramFiles%)
        let expanded_cmd = expand_env_strings(&raw_cmd);

        // 3. Replace target path tokens: %1, %V, %L, %W, %v
        let mut final_cmd = expanded_cmd;
        let mut replaced = false;

        for token in &[
            "\"%1\"", "\"%V\"", "\"%L\"", "\"%W\"", "\"%v\"", "\"%v.\"",
            "%1", "%V", "%L", "%W", "%v.", "%v"
        ] {
            if final_cmd.contains(token) {
                final_cmd = final_cmd.replace(token, &format!("\"{}\"", target));
                replaced = true;
            }
        }

        if !replaced && !target.is_empty() {
            final_cmd = format!("{} \"{}\"", final_cmd.trim(), target);
        }

        tracing::info!("Executing Windows context command: {}", final_cmd);

        // 4. Determine working directory
        let target_p = Path::new(&target);
        let work_dir = if target_p.is_dir() {
            Some(target.clone())
        } else if let Some(parent) = target_p.parent() {
            Some(parent.to_string_lossy().to_string())
        } else {
            None
        };

        // 5. Execute via CreateProcessW, with ShellExecuteW as fallback
        match spawn_process_wide(&final_cmd, work_dir.as_deref()) {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::warn!("CreateProcessW failed ({}), attempting ShellExecute fallback...", e);
                spawn_shellexecute_wide(&final_cmd, work_dir.as_deref())
            }
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = (command_or_id, target_path);
        Err("Windows context menu is only supported on Windows".into())
    }
}

const WIN_E_CLSID_SUBKEY: &str = r"Software\Classes\CLSID\{52205fd8-5dfb-447d-801a-d0b52f2e83e1}";
const WIN_E_CMD_SUBKEY: &str = r"Software\Classes\CLSID\{52205fd8-5dfb-447d-801a-d0b52f2e83e1}\shell\opennewwindow\command";

pub fn is_win_e_replacement_enabled_impl() -> bool {
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::*;
        use winreg::RegKey;
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        if let Ok(cmd_key) = hkcu.open_subkey(WIN_E_CMD_SUBKEY) {
            let val: Result<String, _> = cmd_key.get_value("");
            if let Ok(cmd) = val {
                return !cmd.trim().is_empty();
            }
        }
    }
    false
}

pub fn set_win_e_replacement_impl(enabled: bool) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::*;
        use winreg::RegKey;
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);

        if enabled {
            let exe = std::env::current_exe().map_err(|e| format!("Failed to get exe path: {}", e))?;
            let (cmd_key, _) = hkcu.create_subkey(WIN_E_CMD_SUBKEY)
                .map_err(|e| format!("Failed to create registry key: {}", e))?;
            let cmd_str = format!("\"{}\"", exe.display());
            cmd_key.set_value("", &cmd_str)
                .map_err(|e| format!("Failed to set command value: {}", e))?;
            cmd_key.set_value("DelegateExecute", &"")
                .map_err(|e| format!("Failed to set DelegateExecute: {}", e))?;
            tracing::info!("Win+E shortcut replacement enabled pointing to: {}", cmd_str);
            Ok(())
        } else {
            let _ = hkcu.delete_subkey_all(WIN_E_CLSID_SUBKEY);
            tracing::info!("Win+E shortcut replacement disabled");
            Ok(())
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = enabled;
        Err("Win+E replacement is only supported on Windows".into())
    }
}
