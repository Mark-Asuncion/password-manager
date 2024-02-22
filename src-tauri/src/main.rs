// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
mod tests;
mod file;
mod m_openssl;
mod errors;
mod account;
mod global;
mod frontend;

fn main() {
    tauri::Builder::default()
        .manage(global::Global{
            key_iv: Mutex::new(Default::default()),
            accounts: Mutex::new(Default::default())
        })
        .invoke_handler(tauri::generate_handler![
            frontend::create_key,
            frontend::load_key,
            frontend::skip_setup_page,
            frontend::load_runtime,
            frontend::get_accounts,
            frontend::update_account,
            frontend::save_accounts,
            frontend::add_account,
            frontend::remove_account,
            frontend::append_account,
            frontend::search,
            frontend::export
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
