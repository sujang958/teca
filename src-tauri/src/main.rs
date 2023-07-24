// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, Size, LogicalSize};
use device_query::{DeviceQuery, DeviceState, Keycode};
use std;

#[derive(Clone, serde::Serialize)]
struct Payload {
  key: String,
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

      let settings_window = tauri::WindowBuilder::new(
        app,
        "settings",
        tauri::WindowUrl::App("/settings.html".into()),
      ).build()?;

      let _ = settings_window.set_title("Teca settings");
      let _ = settings_window.set_size(Size::Logical(LogicalSize { width: 400.0, height: 700.0 }));
    
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

