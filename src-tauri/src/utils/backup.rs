use std::{io, path::{PathBuf, Path}, fs::{create_dir_all, self}, collections::VecDeque};

use tauri::AppHandle;

use crate::{account::Account, error};

use super::{mcsv::open_csv_writer, FileNames, get_local_time_str, MFile::{self, open_read}};

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
    if records.is_empty() {
        return Ok(());
    }
    if !bak_dir.exists() {
        create_dir_all(&bak_dir)?;
    }

    let suffix = get_local_time_str();
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

pub fn export_to_tar(data_path: PathBuf, to: &mut PathBuf) -> io::Result<()> {
    if let None = to.extension() {
        to.set_extension("tar");
    }
    let tar_file = MFile::open_write(&to)?;
    let mut builder = tar::Builder::new(tar_file);

    let mut keyiv_f = {
        let mut fname = data_path.clone();
        fname.push(FileNames::KEYIV);
        let f = open_read(&fname)?;
        f
    };

    let mut acc_f = {
        let mut fname = data_path.clone();
        fname.push(FileNames::ACC_LIST);
        let f = open_read(&fname)?;
        f
    };

    let bak_d = {
        let mut fname = data_path.clone();
        fname.push(FileNames::BAK_ACC_D);
        fname
    };

    builder.append_file(Path::new(FileNames::KEYIV), &mut keyiv_f)?;
    builder.append_file(Path::new(FileNames::ACC_LIST), &mut acc_f)?;
    builder.append_dir_all(Path::new(FileNames::BAK_ACC_D), &bak_d)?;
    builder.finish()
}

#[tauri::command]
pub fn create_archive_tar(mut path: PathBuf, handle: AppHandle) -> Result<String, String> {
    let data_dir = handle.path_resolver()
        .app_data_dir()
        .expect(error::DATA_DIR_NOT_EXIST);

    if let Err(e) = export_to_tar(data_dir, &mut path) {
        dbg!(&e);
        return Err(e.to_string());
    }
    let p = path.to_string_lossy().to_string();
    Ok(p)
}
