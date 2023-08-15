// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use device_query::{DeviceQuery, DeviceState, Keycode};
use std;

#[derive(Clone, serde::Serialize)]
struct Payload {
  key: String,
}

#[tauri::command]
fn close_all(app_handle: tauri::AppHandle) {
  app_handle.exit(0);
}

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let main_window = app.get_window("main").unwrap();

      std::thread::spawn(move || {
        let device_state = DeviceState::new();
        let mut past_keys: Vec<Keycode> = Vec::new();

        loop {
          let pressed_keys: Vec<Keycode> = device_state.get_keys();
          
          let detached_keys = past_keys.iter().filter(|key| !pressed_keys.contains(key));

          for key in detached_keys {
            let _ = main_window.emit("keyUp", Payload {key: key.to_string()});
          }
          for key in pressed_keys.iter() {
            let _ = main_window.emit("keyDown", Payload {key: key.to_string()});
          }

          past_keys = pressed_keys.clone();
        }
      });
    
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![close_all])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

