use std::{io, path::{PathBuf, Path}, fs::{create_dir_all, self}, collections::VecDeque};

use crate::account::Account;

use super::{mcsv::open_csv_writer, FileNames};

fn _list_dir(dir: &Path) -> io::Result<Vec<PathBuf>> {
    if !dir.is_dir() { return Ok(vec![]); }
    let mut paths: Vec<PathBuf> = vec![];
    for path in fs::read_dir(dir)? {
        let path = path?.path();
        if path.is_dir() { continue; }
        paths.push(path);
    }

    paths.sort();
    Ok(paths)
}

fn limit_bak(bak_dir: &Path) {
    let files = _list_dir(bak_dir).unwrap_or_default();
    if files.is_empty() {
        return;
    }
    let mut list = VecDeque::from(files);
    while list.len() > 5 {
        let rm = list.pop_front().unwrap();
        fs::remove_file(rm).unwrap_or_default();
    }
}

pub fn backup_accounts(mut bak_dir: PathBuf, records: &[Account]) -> io::Result<()> {
    if !bak_dir.exists() {
        create_dir_all(&bak_dir)?;
    }

    let suffix = chrono::Local::now()
        .format("%Y-%m-%d-%H-%M");
    let fname = format!("{}{}", FileNames::TMP_ACC_LIST, suffix);
    bak_dir.push(&fname);
    let mut writer = open_csv_writer(&bak_dir)?;
    for record in records {
        if let Err(e) = writer.serialize(record) {
            return Err( io::Error::new(io::ErrorKind::Other, e.to_string().as_str()) );
        }
    }

    if let Some(p) = bak_dir.parent() {
        limit_bak(p);
    }
    Ok(())
}


