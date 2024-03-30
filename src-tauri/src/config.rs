use crate::file;
use std::io::{self, Read, Write};
use std::path::PathBuf;


#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[allow(non_snake_case)]
pub struct Config {
    pub useWorkspace: bool,
    pub workspace: String
}

impl Default for Config {
    fn default() -> Self {
        Self {
            useWorkspace: Default::default(),
            workspace: Default::default()
        }
    }
}

impl Config {
    pub fn set_workspace(&mut self, path: String) {
        self.useWorkspace = true;
        self.workspace = path;
    }
    pub fn get_workspace(&self) -> Option<String> {
        if !self.useWorkspace || self.workspace.is_empty() {
            return None;
        }
        Some(self.workspace.clone())
    }

    pub fn path() -> io::Result<PathBuf> {
        let mut p_config = file::uconfig_path()?;
        p_config.push(file::constants::F_CONFIG);
        Ok(p_config)
    }

    pub fn load() -> io::Result<Self> {
        let p_config = Self::path()?;
        let mut f_config = file::open(&p_config)?;
        let mut buf: Vec<u8> = Vec::new();
        f_config.read_to_end(&mut buf)?;
        let str_buf = std::str::from_utf8(&buf).unwrap_or_default();
        if str_buf.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::Other, "Error Occured reading config")
            );
        }
        let config: Config = serde_json::from_str(str_buf)?;
        Ok(config)
    }

    pub fn write(&self) -> io::Result<()> {
        use std::ffi::OsStr;
        let f_config = Self::path()?;
        let mut f_config_tmp = f_config.clone();
        let tmp_filename = f_config_tmp.file_name().unwrap_or(OsStr::new("tmp")).to_str().unwrap();
        let tmp_filename = format!(".{}", tmp_filename);
        f_config_tmp.set_file_name( OsStr::new( &tmp_filename ) );
        let mut file = file::open(&f_config_tmp)?;
        let json = serde_json::to_string_pretty(self)?;
        file.write_all(json.as_bytes())?;
        std::fs::copy(&f_config_tmp, f_config)?;
        std::fs::remove_file(&f_config_tmp)?;
        Ok(())
    }

    pub fn exists() -> bool {
        let p_config = file::uconfig_path();
        match p_config {
            Ok(mut v) => {
                v.push(file::constants::F_CONFIG);
                return v.exists();
            },
            Err(e) => {
                println!("config::exists:: {:?}", e);
                return false;
            }
        }
    }
}
