use serde::{Deserialize, Serialize};
use tauri::State;
use std::sync::Mutex;

use crate::device::DeviceManager;

#[derive(Serialize, Deserialize)]
pub struct DeviceInfo {
    id: String,
    name: String,
    connected: bool,
}

#[tauri::command]
pub async fn ping() -> Result<String, String> {
    Ok("Pong".to_string())
}

#[tauri::command]
pub async fn list_devices(
    device_manager: State<Mutex<DeviceManager>>
) -> Result<Vec<DeviceInfo>, String> {
    let manager = device_manager.lock().unwrap();
    Ok(manager.list_devices())
}

#[tauri::command]
pub async fn read_nand_id(
    device_id: String,
    device_manager: State<Mutex<DeviceManager>>
) -> Result<Vec<u8>, String> {
    let manager = device_manager.lock().unwrap();
    manager.read_nand_id(&device_id)
}

#[tauri::command]
pub async fn dump_nand(
    device_id: String,
    start_page: u32,
    num_pages: u32,
    device_manager: State<Mutex<DeviceManager>>
) -> Result<Vec<u8>, String> {
    let manager = device_manager.lock().unwrap();
    manager.dump_nand(&device_id, start_page, num_pages)
}

