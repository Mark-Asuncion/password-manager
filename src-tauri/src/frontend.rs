use crate::file;
use crate::m_openssl;
use crate::errors;
use crate::global::ACCOUNTS;
use crate::global::KEY_N_IV;
use crate::account::Account;
use serde_json;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use tauri::api::dialog;

#[tauri::command]
pub fn create_key() -> String {
    let mut s = file::udata_path().unwrap();
    s.push(file::constants::F_KEY);
    let path = s.as_path();
    if path.exists() && path.is_file() {
        return errors::json_error("file already exists");
    }
    println!("create_key():: Creating Key");
    let key: Vec<u8>;
    let iv: Vec<u8>;
    match m_openssl::gen_random_bytes() {
        Err(e) => {
            errors::show_error(e.to_string().as_str());
            return errors::json_error(e.to_string().as_str());
        },
        Ok(v) => key = v,
    }
    match m_openssl::gen_random_bytes() {
        Err(e) => {
            errors::show_error(e.to_string().as_str());
            return errors::json_error(e.to_string().as_str());
        },
        Ok(v) => iv = v,
    }

    match file::open(s.as_path()) {
        Ok(mut file) => {
            file.write_all(key.as_slice()).unwrap();
            file.write_all(iv.as_slice()).unwrap();
        },
        Err(e) => {
            errors::show_error(e.to_string().as_str());
            return errors::json_error(e.to_string().as_str());
        },
    }
    r#"{"ok":true}"#.to_string()
}

#[tauri::command]
pub fn load_key() -> String {
    let confirm_dialog = dialog::MessageDialogBuilder::new("Confirm", "A key already exists, do you want to overwrite it?")
        .kind(dialog::MessageDialogKind::Info)
        .buttons(dialog::MessageDialogButtons::YesNo);

    dialog::FileDialogBuilder::new()
        .set_title("Choose A key file")
        .pick_file(|file_path| {
            println!("load_key():: {:?}", file_path.clone().unwrap_or(PathBuf::new()));

            if let None = file_path {
                return;
            }

            let file_path = file_path.unwrap();
            let f_key_path = {
                let mut f = file::udata_path().unwrap();
                f.push(file::constants::F_KEY);
                f
            };

            if file_path == f_key_path { return; }
            if f_key_path.exists() {
                confirm_dialog.show(|option| {
                    if !option { return }
                    let file_path = file_path;
                    let f_key_path = f_key_path;

                    let p_bak = {
                        let mut p_bak = file::udata_path().unwrap();
                        p_bak.push(file::constants::D_BACKUP);
                        p_bak.push(file::constants::D_BACKUP_KEY);
                        p_bak
                    };

                    if let Err(e) = file::backup(f_key_path.as_path(), p_bak.as_path()) {
                        errors::show_error(e.to_string().as_str());
                        return;
                    }
                    if let  Err(e) = fs::copy(file_path.as_path(), f_key_path.as_path()) {
                        errors::show_error(e.to_string().as_str());
                        return;
                    }
                });
            }
            else {
                if let  Err(e) = fs::copy(file_path.as_path(), f_key_path.as_path()) {
                    errors::show_error(e.to_string().as_str());
                };
            }
        });
    r#"{"ok":true}"#.to_string()
}

#[tauri::command]
pub fn load_accounts() -> String {

    dialog::FileDialogBuilder::new()
        .set_title("Account File (csv)")
        .add_filter("CSV document", &[ "csv" ])
        .pick_file(|file_path| {
            println!("load_accounts():: {:?}", file_path.clone().unwrap_or(PathBuf::new()));

            if let None = file_path {
                return;
            }

            let file_path = file_path.unwrap();
            let f_acc_path = {
                let mut f = file::udata_path().unwrap();
                f.push(file::constants::F_ACCOUNT);
                f
            };

            if file_path == f_acc_path { return; }
            if f_acc_path.exists() {
                dialog::MessageDialogBuilder::new("Confirm", "A Account file already exists, do you want to overwrite it?")
                    .kind(dialog::MessageDialogKind::Info)
                    .buttons(dialog::MessageDialogButtons::YesNo)
                    .show(|option| {
                        if !option { return }
                        let file_path = file_path;
                        let f_acc_path = f_acc_path;

                        let p_bak = {
                            let mut p_bak = file::udata_path().unwrap();
                            p_bak.push(file::constants::D_BACKUP);
                            p_bak.push(file::constants::D_BACKUP_ACCOUNT);
                            p_bak
                        };

                        if let Err(e) = file::backup(f_acc_path.as_path(), p_bak.as_path()) {
                            errors::show_error(e.to_string().as_str());
                            return;
                        }
                        if let  Err(e) = fs::copy(file_path.as_path(), f_acc_path.as_path()) {
                            errors::show_error(e.to_string().as_str());
                            return;
                        }
                    });
            }
            else {
                if let  Err(e) = fs::copy(file_path.as_path(), f_acc_path.as_path()) {
                    errors::show_error(e.to_string().as_str());
                };
            }
        });
    r#"{"ok":true}"#.to_string()
}

#[tauri::command]
pub fn skip_setup_page() -> bool {
    let mut s = file::udata_path().unwrap();
    s.push(file::constants::F_KEY);
    if s.exists() { return true; }
    return false;
}

#[tauri::command]
pub fn load_runtime() -> String {
    let f_key = {
        let mut f = file::udata_path().unwrap();
        f.push(file::constants::F_KEY);
        f
    };
    match file::open(f_key.as_path()) {
        Ok(mut v) => {
            unsafe {
                KEY_N_IV = m_openssl::open_key(&mut v);
                if KEY_N_IV.0.is_empty() {
                    errors::show_error("key is empty");
                    return errors::json_error("key is empty");
                }
            }
        },
        Err(e) => {
            errors::show_error(e.to_string().as_str());
            return errors::json_error(e.to_string().as_str());
        }
    }

    let f_acc = {
        let mut f = file::udata_path().unwrap();
        f.push(file::constants::F_ACCOUNT);
        f
    };
    match file::read_csv(f_acc.as_path()) {
        Ok(v) => {
            unsafe {
                ACCOUNTS = v;
            }
        }
        Err(e) => {
            errors::show_error(e.to_string().as_str());
            return errors::json_error(e.to_string().as_str());
        }
    }
    r#"{"ok":true}"#.to_string()
}

#[tauri::command]
pub fn get_accounts() -> String {
    unsafe {
        let accounts = &ACCOUNTS;
        let mut ret: Vec<serde_json::Value> = vec![];
        for account in accounts {
            match account.get_pass_decrypted(&KEY_N_IV.0, &KEY_N_IV.1) {
                Ok(pass) => {
                    let j = serde_json::json!({
                    "username": account.username,
                    "link": account.link,
                    "password": pass
                    });
                    ret.push(j);
                },
                Err(e) => {
                    errors::show_error(format!("Cannot decrypt password of \"{}\'\n{}", &account.username,e).as_str());
                    let j = serde_json::json!({
                    "username": account.username,
                    "link": account.link,
                    "password": account.password
                    });
                    ret.push(j);
                }
            }
        }
        match serde_json::to_string(&ret) {
            Ok(v) => {
                // println!("{}", v);
                return v;
            },
            Err(e) => {
                errors::show_error(e.to_string().as_str());
                return errors::json_error(e.to_string().as_str());
            }
        }
    }
}

#[tauri::command]
pub fn update_account(row: usize, key: String, val: String) {
    println!("update_account:: {} {} {}", row, key ,val);
    unsafe {
        let val = val.trim().to_string();
        if let Some(account) = ACCOUNTS.get_mut(row) {
            match key.as_str() {
                "username" => account.username = val,
                "link" => account.link = val,
                "password" => {
                    match m_openssl::encrypt(&KEY_N_IV.0, &KEY_N_IV.1, &val) {
                        Ok(v) => account.password = v,
                        Err(e) => errors::show_error(e.to_string().as_str())
                    }
                },
                _ => {}
            }
        }
    }
}

#[tauri::command]
pub fn save_accounts() {
    unsafe {
        let accounts = &ACCOUNTS;
        let mut s = file::udata_path().unwrap();
        s.push(file::constants::F_ACCOUNT);

        if let Err(e) = file::write_csv(
            s.as_path(),
            accounts.as_slice(),
            &KEY_N_IV.0,
            &KEY_N_IV.1
        ) {
            errors::show_error(e.to_string().as_str());
        }
    }
}

#[tauri::command]
pub fn add_account(username: String, link: String, password: String) -> String {
    if username.is_empty() || password.is_empty() {
        return r#"{"error":"username or password is empty"}"#.to_string();
    }
    unsafe {
        let accounts = &mut ACCOUNTS;
        match m_openssl::encrypt(&KEY_N_IV.0, &KEY_N_IV.1, &password) {
            Ok(v) => {
                accounts.push(
                    Account::new(username.as_str(),link.as_str(),v.as_str())
                );
                println!("add_account():: {:?}", accounts.last().unwrap());
                let res = serde_json::json!({
                    "username": accounts.last().unwrap().username,
                    "link": accounts.last().unwrap().link,
                    "password": password
                });
                match serde_json::to_string(&res) {
                    Ok(v) => return v,
                    Err(e) => {
                        errors::show_error(e.to_string().as_str());
                        return errors::json_error(e.to_string().as_str());
                    }
                }
            },
            Err(e) => {
                errors::show_error(e.to_string().as_str());
                return errors::json_error(e.to_string().as_str());
            }
        }
    }
}

#[tauri::command]
pub fn get_pass(row: usize) -> String {
    unsafe {
        let accounts = &ACCOUNTS;
        let account = accounts.get(row).unwrap();
        match account.get_pass_decrypted(&KEY_N_IV.0, &KEY_N_IV.1) {
            Err(e) => {
                errors::show_error(e.as_str());
                return errors::json_error(e.as_str());
            },
            Ok(v) => format!("{{\"password\":\"{}\"}}", v),
        }
    }
}

#[tauri::command]
pub fn remove_account(row: usize) {
    unsafe {
        assert!(row < ACCOUNTS.len());
        let removed = ACCOUNTS.remove(row);
        println!("remove_account:: {:?}", removed);
    }
}

#[tauri::command]
pub fn append_account(path: String) -> String {
    let file_path = PathBuf::from(path);
    if let Ok(v) = file::read_csv(file_path.as_path()) {
        let mut res: Vec<Account> = vec![];
        unsafe {
            for mut acc in v {
                let pass = acc.password.clone();
                match m_openssl::encrypt(&KEY_N_IV.0, &KEY_N_IV.1, &pass) {
                    Err(e) => errors::show_error(e.to_string().as_str()),
                    Ok(v) => {
                        acc.password = v;
                    }
                }
                res.push(acc);
            }
            if !res.is_empty() {
                ACCOUNTS.append(&mut res);
            }
        }
    }
    return get_accounts();
}
