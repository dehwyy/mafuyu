use base64::prelude::*;
use base64::engine::general_purpose::STANDARD;
use std::{fs::File, io::{self, Write}, path::Path};
use logger::info;


pub struct CDNFs {
    base_dir: String,
}

impl CDNFs {
    pub fn new() -> Self {
        Self {
            base_dir: ".cdn".to_string(),
        }
    }

    // todo: handle errors
    pub fn save_image(&self, filename: &str, buffer: String, ext: String) -> Result<(), String> {
        let v = STANDARD.decode(buffer).map_err(|err| err.to_string())?;

        let mut file = Self::new_rw_file(Path::new(&self.get_filepath(filename, &ext)))
            .map_err(|err| err.to_string())?;

        file.write(&v).map_err(|err| err.to_string())?;

        Ok(())
    }

    fn new_rw_file(path: &Path) -> Result<File, io::Error> {
        File::options().read(true).write(true).create_new(true).open(path)
    }

    fn get_filepath(&self, filename: &str, ext: &str) -> String {
        format!("./{}/static/{}.{}", self.base_dir, filename, ext)
    }
}