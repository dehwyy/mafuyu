use base64::prelude::*;
use std::fs::File;
use std::io::Write;


pub struct CDNFs {
    base_dir: String,
}

impl CDNFs {
    pub fn new() -> Self {
        Self {
            base_dir: ".cdn".to_string(),
        }
    }

    pub fn save_image(&self, filename: &str, buffer: Vec<u8>, ext: String) -> Result<(), String> {
        let v = BASE64_STANDARD.decode(buffer).map_err(|err| err.to_string())?;

        let path = format!("./{}/static/{}.{}", self.base_dir, filename, ext);

        let mut file = File::options().read(true).write(true).open(std::path::Path::new(&path)).map_err(|err| err.to_string())?;
        file.write(&v).map_err(|err| err.to_string())?;

        Ok(())
    }
}