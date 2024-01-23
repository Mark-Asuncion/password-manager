use crate::account::Account;
use std::collections::VecDeque;
use std::io;
use std::path::{Path, PathBuf};
use std::fs::{self, File, OpenOptions};
use homedir::get_my_home;
use csv::{Writer, Reader};

#[allow(unused)]
pub mod constants {
    // TODO include windows
    // unix like
    pub const CONFIG: &str = "/.config/password-manager";
    pub const DATA: &str = "/.local/share/password-manager/user";
    pub const F_ACCOUNT: &str = "acc.csv";
    pub const F_KEY: &str = "aesk";
    pub const D_BACKUP: &str = ".bak";
    pub const D_BACKUP_KEY: &str = "key";
    pub const D_BACKUP_ACCOUNT: &str = "account";
    pub const BAK_LIMIT: usize = 5;
}

fn _list_dir_files(path_dir: &Path) -> io::Result<Vec<PathBuf>> {
    if !path_dir.is_dir() { return Ok(vec![]); }
    let mut paths: Vec<PathBuf> = vec![];
    for path in fs::read_dir(path_dir)? {
        let path = path?.path();
        if path.is_dir() { continue; }
        paths.push(path);
    }

    paths.sort();
    Ok(paths)
}

fn _create_dir(path: &Path) -> io::Result<()> {
    if !path.exists() {
        fs::create_dir_all(path.to_str().unwrap())?;
    }
    Ok(())
}

fn userhome() -> String {
    get_my_home().unwrap().unwrap()
        .to_str().unwrap().to_string()
}

fn _limit_backup(path_dir: &Path) -> io::Result<()> {
    let list = _list_dir_files(path_dir)?;
    let mut list = VecDeque::from(list);
    while list.len() > constants::BAK_LIMIT {
        let rm = list.pop_front().unwrap();
        fs::remove_file(rm)?;
    }
    Ok(())
}

#[allow(dead_code)]
pub fn uconfig_path() -> io::Result<PathBuf> {
    let path =  userhome() + constants::CONFIG;
    let p = PathBuf::from(path.as_str());
    _create_dir(p.as_path())?;
    Ok(p)
}

pub fn udata_path() -> io::Result<PathBuf> {
    let path =  userhome() + constants::DATA;
    let p = PathBuf::from(path.as_str());
    _create_dir(p.as_path())?;
    Ok(p)
}

pub fn backup(path_file: &Path, where_to: &Path) -> io::Result<()> {
    if !path_file.exists() {
        return Ok(());
    }

    if where_to.exists() && !where_to.is_dir() {
        return Err(
            io::Error::new(
                io::ErrorKind::Other,
                "Destination is not a directory"
            )
        );
    }

    if path_file.is_dir() {
        return Err(
            io::Error::new(
                io::ErrorKind::Other,
                "Cannot Backup a Directory"
            )
        );
    }

    _create_dir(where_to)?;
    let to = {
        let fname = path_file.file_name().unwrap()
            .to_str().unwrap()
            .to_string();
        let suffix = chrono::Local::now()
            .format("%Y-%m-%d-%H-%M");
        let name = format!("{}.{}", fname, suffix);
        let to = where_to.join(name);
        to
    };
    // println!("backup():: from {:?} to {:?}", path_file, to);
    fs::copy(path_file, to)?;
    _limit_backup(where_to)?;
    Ok(())
}

pub fn open(path_file: &Path) -> io::Result<File> {
    File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(path_file)
}

pub fn read_csv(path_file: &Path) -> io::Result<Vec<Account>> {
    if !path_file.exists() { return Ok(vec![]); }
    let r: Reader<File>;
    match Reader::from_path(path_file) {
        Ok(v) => r = v,
        Err(e) => return Err( io::Error::new(io::ErrorKind::Other, e.to_string()))
    }
    let iter = r.into_records();
    let mut accs: Vec<Account> = Vec::new();

    for string_record in iter {
        if let Ok(v) = string_record {
            // println!("read_csv():: string_record {:?}",v);
            if v.len() <= 3 {
                accs.push(v.deserialize(None).unwrap());
            }
        }
    }
    // println!("read_csv():: accs {:?}",accs);
    Ok(accs)
}

pub fn _open_csv_writer(path_file: &Path) -> io::Result<Writer<File>> {
    match OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path_file) {
            Ok(v) => return Ok(Writer::from_writer(v)),
            Err(e) => return Err(e)
    };
}

pub fn _backup_account(key: &[u8], iv: &[u8], path_file: &Path, where_to: &Path) {
    if !path_file.exists() { return; }
    if let Err(_) = _create_dir(where_to) {
        return;
    }

    let accounts = read_csv(path_file).unwrap_or(vec![]);
    if accounts.is_empty() { return; }
    let f_name = {
        let fname = path_file.file_name().unwrap()
            .to_str().unwrap()
            .to_string();
        let suffix = chrono::Local::now()
            .format("%Y-%m-%d-%H-%M");
        format!("{}.{}", fname, suffix)
    };
    let to = where_to.join(f_name);
    if let Ok(mut writer) = _open_csv_writer(to.as_path()) {
        writer.write_record(&Account::csv_header()).unwrap_or_default();
        for account in accounts {
            let password = account.get_pass_decrypted(key, iv).unwrap_or(account.password.clone());
            // println!("_backup_account:: {:?}",&[account.username.as_str(), account.link.as_str(),password.as_str()]);
            writer.write_record(&[account.username.as_str(), account.link.as_str(),password.as_str()]).unwrap_or_default();
        }
    }
    _limit_backup(where_to).unwrap_or_default();
}

pub fn write_csv(path_file: &Path, records: &[Account], key: &[u8], iv: &[u8]) -> io::Result<()> {
    {
        let bak_dir = {
            let mut s = udata_path().unwrap();
            s.push(constants::D_BACKUP);
            s.push(constants::D_BACKUP_ACCOUNT);
            s
        };
        _backup_account(key, iv, path_file, bak_dir.as_path());
    }

    let mut writer = _open_csv_writer(path_file)?;
    for record in records {
        if let Err(e) = writer.serialize(record) {
            println!("write_csv()::Error writing Aborting {}", e.to_string());
            return Err( io::Error::new(io::ErrorKind::Other, e.to_string().as_str()) );
        }
    }
    Ok(())
}
