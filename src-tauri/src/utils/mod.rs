pub mod mcsv;
pub mod backup;

#[allow(non_snake_case)]
pub mod FileNames {
    pub const KEYIV:        &str = "aesk";
    pub const ACC_LIST:     &str = "accs.csv";
    pub const TMP_ACC_LIST: &str = ".accs";
    pub const BAK_ACC_D:    &str = ".bak";
}

#[allow(non_snake_case)]
pub mod MFile {
    use std::{io::{self}, fs::File, path::Path};

    pub fn open_read(path: &Path) -> io::Result<File> {
        File::options()
            .read(true)
            .open(path)
    }

    pub fn open_write(path: &Path) -> io::Result<File> {
        File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
    }
}

pub fn get_local_time_str() -> String {
    let suffix = chrono::Local::now()
        .format("%Y-%m-%d-%H-%M");
    suffix.to_string()
}
