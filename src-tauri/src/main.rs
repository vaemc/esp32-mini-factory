// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serialport::available_ports;
use std::env;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_serial_port_list() -> String {
    let port_info_list = available_ports().unwrap();
    let port_list: Vec<String> = port_info_list
        .iter()
        .map(|x| x.port_name.to_string())
        .collect();
    let json_data = serde_json::to_string(&port_list).unwrap();
    return json_data;
}

#[tauri::command]
fn get_current_dir() -> String {
    let path = env::current_dir().unwrap();
    return path.display().to_string();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_serial_port_list,
            get_current_dir,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
