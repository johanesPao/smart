use tauri::Manager;
use window_vibrancy::apply_mica;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn welcome() -> String {
    format!("Welcome to the SOSCO Merchandising Analytics and Reporting Tool")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let splashscreen_window = app.get_webview_window("splashscreen").unwrap();
            let main_window = app.get_webview_window("main").unwrap();
            // Perform initialization tasks here
            tauri::async_runtime::spawn(async move {
                // Initialize
                println!("Initializing...");
                std::thread::sleep(std::time::Duration::from_secs(2));
                println!("Done initializing.");

                // Close splashscreen and display main window
                splashscreen_window.close().unwrap();

                // Apply acrylic on main_window
                #[cfg(target_os = "windows")]
                apply_mica(&main_window, Some(true))
                    .expect("Unsupported platform! 'apply_acrylic' is only supported on Windows");
                main_window.show().unwrap();
            });

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![welcome])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
