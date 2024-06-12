// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::create_dir_all;

mod state;
mod account;
mod crypt;
mod tests;
mod error;
mod utils;

fn main() {
    tauri::Builder::default()
        .manage(state::MState::default())
        .setup(|app| {
            let data_dir = app.path_resolver().app_data_dir()
                .expect("APP_DATA_DIR failed to obtain");
            create_dir_all(data_dir)
                .expect("APP_DATA_DIR failed to create");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            state::save,
            state::update_account,
            state::add_account,
            state::get_accounts
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
