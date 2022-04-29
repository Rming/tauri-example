#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;

fn main() {
  tauri::Builder::default()
    .setup(|app|{
      let win = app.get_window("main").unwrap();
      win.show().unwrap();
      win.open_devtools();
      Ok(())
    })
    .build(tauri::generate_context!())
    .expect("error while running tauri application")
    .run(|app, e| match e {
      tauri::RunEvent::WindowEvent { label, event, .. } => {
        // println!("{:?}", event);
        match event {
          tauri::WindowEvent::CloseRequested { api, ..} =>{
            api.prevent_close();
            let win = app.get_window(&label).unwrap();
            win.hide().unwrap();
          },
          _ => {}
        }
      }
      _ => {}
    })
}
