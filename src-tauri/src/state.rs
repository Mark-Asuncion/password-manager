use std::io;
use std::path::PathBuf;
use std::{sync::Mutex, io::Write, fs::File, path::Path};
use tauri::{AppHandle, State};

use crate::account::QueryAccount;
use crate::utils::mcsv::read_csv_plain;
use crate::{account::{Accounts, Account}, error, crypt::{gen_32_bytes, KeyIv, read_key}, utils::{MFile, FileNames, mcsv}};

pub struct MState {
    keyiv:    Mutex<KeyIv>,
    accounts: Mutex<Option<Accounts>>
}

impl Default for MState {
    fn default() -> Self {
        Self {
            keyiv:    Default::default(),
            accounts: Default::default()
        }
    }
}

impl MState {
    pub fn load_keyiv(&self, mut file: File) {
        let keyiv = &mut (*self.keyiv.lock().expect(error::ACQ_STATE_KEYIV));
        *keyiv = read_key(&mut file);
    }

    pub fn load_keyiv_from(&self, buf: Vec<u8>) {
        let keyiv = &mut (*self.keyiv.lock().expect(error::ACQ_STATE_KEYIV));
        *keyiv = KeyIv::from(buf);
    }

    pub fn is_keyiv_loaded(&self) -> bool {
        let keyiv = &(*self.keyiv.lock().expect(error::ACQ_STATE_KEYIV));
        !keyiv.is_empty()
    }

    pub fn is_accounts_loaded(&self) -> bool {
        let accs = &(*self.accounts.lock().expect(error::ACQ_STATE_ACCOUNTS));
        if let None = accs {
            return false;
        }
        true
    }

    fn accs_push(&self, v: Account) {
        let accs = &mut (*self.accounts.lock().expect(error::ACQ_STATE_ACCOUNTS));
        let accs = accs.as_mut().unwrap();
        accs.push(v);
    }

    fn accs_update_at(&self, query: QueryAccount, update: QueryAccount) -> Option<()> {
        let accs = &mut (*self.accounts.lock().expect(error::ACQ_STATE_ACCOUNTS));
        let accs = accs.as_mut().unwrap();
        for acc in accs {
            if query.match_count(&acc) >= 2 {
                acc.set_ignore_empty(update.to_account());
                dbg!(&acc);
                return Some(())
            }
        }
        None
    }

    fn accs_save(&self, path: &Path, bak_dir: PathBuf) -> io::Result<()> {
        let keyiv = &(*self.keyiv.lock().expect(error::ACQ_STATE_KEYIV));
        if keyiv.is_empty() {
            return Ok(());
        }
        let accs = &mut (*self.accounts.lock().expect(error::ACQ_STATE_ACCOUNTS));
        let accs = accs.as_mut().unwrap();
        let accs_slice = accs.as_slice();
        mcsv::write_csv(path, accs_slice, &keyiv, bak_dir)
    }

    fn accs_load(&self, path: &Path) {
        let keyiv = &(*self.keyiv.lock().expect(error::ACQ_STATE_KEYIV));
        let accsl = mcsv::read_csv(path, &keyiv).unwrap_or_default();
        let accs = &mut (*self.accounts.lock().expect(error::ACQ_STATE_ACCOUNTS));
        *accs = Some(accsl);
    }

    fn accs_append(&self, path_file: &Path) -> io::Result<()> {
        let accs = &mut (*self.accounts.lock().expect(error::ACQ_STATE_ACCOUNTS));
        let accs = accs.as_mut().unwrap();
        let mut ar = read_csv_plain(path_file)?;
        accs.append(&mut ar);

        Ok(())
    }

    fn accs_delete(&self, query: QueryAccount) {
        let accs = &mut (*self.accounts.lock().expect(error::ACQ_STATE_ACCOUNTS));
        let accs = accs.as_mut().unwrap();
        let mut new_accs = vec![];
        let mut delete1 = false;
        for acc in accs.iter() {
            if query.match_count(&acc) >= 3 && !delete1 {
                delete1 = true;
                continue;
            }
            new_accs.push(acc.clone());
        }
        *accs = new_accs;
        dbg!(&accs);
    }
}

fn _load_keyiv(mut path: PathBuf, state: &State<MState>) -> Result<(), String>  {
    if !state.is_keyiv_loaded() {
        path.push(FileNames::KEYIV);
        if !path.exists() {
            let keyiv = gen_32_bytes();
            if let Err(e) = keyiv {
                return Err(e.to_string());
            }
            let keyiv = keyiv.unwrap();
            let file = MFile::open_write(&path);
            if let Err(e) = file {
                return Err(e.to_string());
            }
            let mut file = file.unwrap();
            if let Err(e) = file.write_all(&keyiv) {
                return Err(e.to_string());
            };
            state.load_keyiv_from(keyiv);
        }
        else {
            let file = MFile::open_read(&path);
            if let Err(e) = file {
                return Err(e.to_string());
            }
            let file = file.unwrap();
            state.load_keyiv(file)
        }
    }
    Ok(())
}

#[tauri::command]
pub fn save(handle: AppHandle, state: State<MState>) -> Result<(), String> {
    let mut data_dir = handle.path_resolver()
        .app_data_dir()
        .expect(error::DATA_DIR_NOT_EXIST);
    let ddir = data_dir.clone();

    if let Err(e) = _load_keyiv(data_dir.clone(), &state) {
        return Err(e);
    }

    data_dir.push(FileNames::ACC_LIST);
    if let Err(e) = state.accs_save(&data_dir, ddir) {
        return Err(e.to_string());
    }

    Ok(())
}

#[tauri::command]
pub fn get_accounts(query: Option<QueryAccount>, handle: AppHandle, state: State<MState>) -> Vec<Account> {
    let mut data_dir = handle.path_resolver()
        .app_data_dir()
        .expect(error::DATA_DIR_NOT_EXIST);

    if let Err(e) = _load_keyiv(data_dir.clone(), &state) {
        dbg!(e);
        return vec![];
    }

    data_dir.push(FileNames::ACC_LIST);
    if data_dir.exists() {
        if !state.is_accounts_loaded() {
            state.accs_load(&data_dir);
        }
    }
    let accs = &mut (*state.accounts.lock().expect(error::ACQ_STATE_ACCOUNTS));
    let accs = accs.as_mut().unwrap();
    if let None = query {
        return accs.clone();
    }
    let query = query.unwrap();
    let mut res = vec![];
    for acc in accs {
        if query.find_count_readonly(&acc) != 0 {
            res.push(acc.clone());
        }
    }
    // dbg!(&res);
    res
}

#[tauri::command]
pub fn update_account(query: QueryAccount, update: QueryAccount, state: State<MState>) {
   state.accs_update_at(query, update);
}

#[tauri::command]
pub fn add_account(v: Account, state: State<MState>) {
    state.accs_push(v);
}

#[tauri::command]
pub fn append_account(path_file: PathBuf, state: State<MState>) -> Result<(), String> {
    if let Err(e) = state.accs_append(&path_file) {
        dbg!(e);
        return Err(error::CSV_WRONG_FORMAT.into());
    }

    Ok(())
}

#[tauri::command]
pub fn delete_account(query: QueryAccount, state: State<MState>) {
    state.accs_delete(query);
}
