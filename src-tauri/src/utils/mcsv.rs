use std::{fs::{OpenOptions, File}, path::{Path, PathBuf}};
use std::io;

use csv::{Writer, Reader};

use crate::{account::{Account, Accounts}, crypt::KeyIv, utils::get_local_time_str};

use super::{FileNames, backup::backup_accounts};

pub fn open_csv_writer(path_file: &Path) -> io::Result<Writer<File>> {
    match OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path_file) {
            Ok(v) => return Ok(Writer::from_writer(v)),
            Err(e) => return Err(e)
    };
}

pub fn write_csv(path_file: &Path, records: &[Account], keyiv: &KeyIv, mut data_dir: PathBuf) -> io::Result<()> {
    {
        data_dir.push(FileNames::BAK_ACC_D);
        if let Err(e) = backup_accounts(data_dir, records) {
            dbg!(e);
        }
    }

    let mut writer = open_csv_writer(path_file)?;
    for record in records {
        let ser = record.as_encrypted(&keyiv);
        if let Err(e) = writer.serialize(ser) {
            println!("[{} utils::mcsv::write_csv]::Error writing Aborting {}", get_local_time_str(), e.to_string());
            return Err( io::Error::new(io::ErrorKind::Other, e.to_string().as_str()) );
        }
    }
    Ok(())
}

pub fn read_csv(path_file: &Path, keyiv: &KeyIv) -> io::Result<Accounts> {
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
            if v.len() <= 3 {
                let a: Account = v.deserialize(None)?;
                let aa = a.as_decrypted(&keyiv);
                if aa.password.is_empty() {
                    println!("[{} utils::mcsv::read_csv]::Failed to decrypt password {:?}", get_local_time_str(), &a);
                }
                accs.push(aa);
            }
        }
    }
    Ok(accs)
}

pub fn read_csv_plain(path_file: &Path) -> io::Result<Accounts> {
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
            if v.len() <= 3 {
                accs.push(v.deserialize(None)?);
            }
        }
    }
    Ok(accs)
}
