// OpenFlash Tauri Backend
use tauri::Manager;

mod command;
mod device;
mod flasher;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                app.get_webview_window("main").unwrap().open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            command::ping,
            command::list_devices,
            command::read_nand_id,
            command::dump_nand,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

