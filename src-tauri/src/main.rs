// main.rs — Tauri entry-point: assembly only, zero platform logic.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    #[cfg(target_os = "windows")]
    {
        if let Ok(exe) = std::env::current_exe() {
            if let Some(parent) = exe.parent() {
                let data_dir = parent.join("Data");
                if data_dir.exists() {
                    let wv2_dir = data_dir.join("WebView2");
                    let _ = std::fs::create_dir_all(&wv2_dir);
                    std::env::set_var("WEBVIEW2_USER_DATA_FOLDER", &wv2_dir);
                }
            }
        }
    }

    files_explorer_lib::tauri_setup::init_logging();
    tracing::info!("Application starting");

    let app = files_explorer_lib::tauri_setup::build_app();
    app.run(|app_handle, event| {
        files_explorer_lib::tauri_setup::run_callback(app_handle, event);
    });
}
