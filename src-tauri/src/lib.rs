use tauri::Manager;

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
                main_window.show().unwrap();
            });
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![welcome])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
