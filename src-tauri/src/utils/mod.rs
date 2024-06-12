pub mod mcsv;
pub mod backup;

pub type Query = (String, String, String);

pub mod FileNames {
    pub const KEYIV: &str = "aesk";
    pub const ACC_LIST: &str = "accs.csv";
    pub const TMP_ACC_LIST: &str = ".accs";
    pub const BAK_ACC_D: &str = ".bak";
}

pub mod MFile {
    use std::{io::{self, Write}, fs::File, path::Path};

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
