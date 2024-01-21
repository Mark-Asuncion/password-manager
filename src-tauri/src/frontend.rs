use crate::file;
use crate::m_openssl;
use crate::errors;
use crate::global::Global;
use crate::account::Account;
use serde_json;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use tauri::api::dialog;
use tauri::State;

#[tauri::command]
pub fn create_key() -> String {
    let mut s = file::udata_path().unwrap();
    s.push(file::constants::F_KEY);
    let path = s.as_path();
    if path.exists() && path.is_file() {
        return errors::json_error("file already exists");
    }
    // println!("create_key():: Creating Key");
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
        .kind(dialog::MessageDialogKind::Warning)
        .buttons(dialog::MessageDialogButtons::YesNo);

    dialog::FileDialogBuilder::new()
        .set_title("Choose A key file")
        .pick_file(|file_path| {
            // println!("load_key():: {:?}", file_path.clone().unwrap_or(PathBuf::new()));

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
pub fn skip_setup_page() -> bool {
    let mut s = file::udata_path().unwrap();
    s.push(file::constants::F_KEY);
    s.exists()
}

#[tauri::command]
pub fn load_runtime(state: State<Global>) -> String {
    let f_key = {
        let mut f = file::udata_path().unwrap();
        f.push(file::constants::F_KEY);
        f
    };
    match file::open(f_key.as_path()) {
        Ok(mut v) => {
            let keyiv = m_openssl::open_key(&mut v);
            if keyiv.0.is_empty() {
                errors::show_error("key is empty");
                return errors::json_error("key is empty");
            }
            *state.key_iv.lock()
                .expect(errors::mutex_lock_error("key and iv").as_str()) = keyiv;
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
            let acc = &mut *state.accounts.lock().expect(errors::mutex_lock_error("accounts").as_str());
            *acc = v;
        }
        Err(e) => {
            errors::show_error(e.to_string().as_str());
            return errors::json_error(e.to_string().as_str());
        }
    }
    r#"{"ok":true}"#.to_string()
}

#[tauri::command]
pub fn get_accounts(state: State<Global>) -> String {
    let accounts = &*state.accounts.lock().expect(errors::mutex_lock_error("accounts").as_str());
    let key_iv = &*state.key_iv.lock().expect(errors::mutex_lock_error("key and iv").as_str());
    let mut ret: Vec<serde_json::Value> = vec![];
    for ( id, account ) in accounts.iter().enumerate() {
        match account.as_json_decrypted(id, &key_iv.0, &key_iv.1) {
            Ok(json) => {
                ret.push(json);
            },
            Err(e) => {
                errors::show_error(format!("Cannot decrypt password of \"{}\'\n{}", &account.username,e).as_str());
                ret.push(account.as_json(id));
            }
        }
        // match account.get_pass_decrypted(&key_iv.0, &key_iv.1) {
        //     Ok(pass) => {
        //         let j = serde_json::json!({
        //         "username": account.username,
        //         "link": account.link,
        //         "password": pass
        //         });
        //         ret.push(j);
        //     },
        //     Err(e) => {
        //         errors::show_error(format!("Cannot decrypt password of \"{}\'\n{}", &account.username,e).as_str());
        //         let j = serde_json::json!({
        //         "username": account.username,
        //         "link": account.link,
        //         "password": account.password
        //         });
        //         ret.push(j);
        //     }
        // }
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

#[tauri::command]
pub fn update_account(id: usize, key: String, val: String,state: State<Global>) {
    // println!("update_account:: {} {} {}", id, key ,val);
    let val = val.trim().to_string();
    let accounts = &mut *state.accounts.lock().expect(errors::mutex_lock_error("accounts").as_str());
    if let Some(account) = accounts.get_mut(id) {
        match key.as_str() {
            "username" => account.username = val,
            "link" => account.link = val,
            "password" => {
                let key_iv = &*state.key_iv.lock().expect(errors::mutex_lock_error("key and iv").as_str());
                match m_openssl::encrypt(&key_iv.0, &key_iv.1, &val) {
                    Ok(v) => account.password = v,
                    Err(e) => errors::show_error(e.to_string().as_str())
                }
            },
            _ => {}
        }
    }
}

#[tauri::command]
pub fn save_accounts(state: State<Global>) {
    let accounts = &*state.accounts.lock().expect(errors::mutex_lock_error("accounts").as_str());
    let key_iv = &*state.key_iv.lock().expect(errors::mutex_lock_error("key and iv").as_str());
    let mut s = file::udata_path().unwrap();
    s.push(file::constants::F_ACCOUNT);

    if let Err(e) = file::write_csv(
        s.as_path(),
        accounts.as_slice(),
        &key_iv.0,
        &key_iv.1
    ) {
        errors::show_error(e.to_string().as_str());
    }
}

#[tauri::command]
pub fn add_account(username: String, link: String, password: String, state: State<Global>) -> String {
    if username.is_empty() || password.is_empty() {
        return r#"{"error":"username or password is empty"}"#.to_string();
    }
    let accounts = &mut *state.accounts.lock().expect(errors::mutex_lock_error("accounts").as_str());
    let key_iv = &*state.key_iv.lock().expect(errors::mutex_lock_error("key and iv").as_str());
    let password = password.trim().to_string();
    match m_openssl::encrypt(&key_iv.0, &key_iv.1, &password) {
        Ok(v) => {
            accounts.push(
                Account::new(username.as_str(),link.as_str(),v.as_str())
            );
            // println!("add_account():: {:?}", accounts.last().unwrap());
            let last = accounts.last().unwrap();
            let res = Account::json(
                accounts.len()-1,
                last.username.as_str(),
                last.link.as_str(),
                password.as_str()
            );
            return serde_json::to_string(&res).unwrap();
        },
        Err(e) => {
            errors::show_error(e.to_string().as_str());
            return errors::json_error(e.to_string().as_str());
        }
    }
}

#[tauri::command]
pub fn remove_account(id: usize, state: State<Global>) {
    let accounts = &mut *state.accounts.lock().expect(errors::mutex_lock_error("accounts").as_str());
    assert!(id < accounts.len());
    #[allow(unused_variables)]
    let removed = accounts.remove(id);
    dbg!(removed);
}

#[tauri::command]
pub fn append_account(path: String, state: State<Global>) {
    let file_path = PathBuf::from(path);
    if let Ok(v) = file::read_csv(file_path.as_path()) {
        let mut res: Vec<Account> = vec![];
        for mut acc in v {
            let pass = acc.password.clone();
            let key_iv = &*state.key_iv.lock().expect(errors::mutex_lock_error("key and iv").as_str());
            match m_openssl::encrypt(&key_iv.0, &key_iv.1, &pass) {
                Err(e) => errors::show_error(e.to_string().as_str()),
                Ok(v) => {
                    acc.password = v;
                }
            }
            res.push(acc);
        }
        if !res.is_empty() {
            let accounts = &mut *state.accounts.lock().expect(errors::mutex_lock_error("accounts").as_str());
            accounts.append(&mut res);
        }
    }
}

#[tauri::command]
pub fn search(val: String, state: State<Global> ) -> String {
    let accs = & *state.accounts.lock().expect("accounts");
    let mut ret = vec![];
    dbg!(&val);
    for (i, acc) in accs.iter().enumerate() {
        if acc.is_match(&val) {
            let key_iv = &*state.key_iv.lock().expect(errors::mutex_lock_error("key and iv").as_str());
            match acc.as_json_decrypted(i, &key_iv.0, &key_iv.1) {
                Ok(v) => {
                    ret.push(v);
                },
                Err(e) => {
                    errors::show_error(e.as_str());
                }
            }
        }
    }
    serde_json::to_string(&ret).unwrap_or_default()
}
