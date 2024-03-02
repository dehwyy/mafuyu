use base64::prelude::*;
use base64::engine::general_purpose::STANDARD;
use std::{fs::File, io::{self, Write}, path::Path};
use std::io::{Cursor, Read};
use image::{DynamicImage, ImageFormat};

use image::io::Reader as ImageReader;
use logger::error;

const BASE_DIR: &str = ".cdn";
pub struct CDNFs;

impl CDNFs {
    pub fn read_image(filename: &str, file_ext: &str) -> io::Result<DynamicImage> {
        let path = Self::get_filepath(filename, file_ext);

        // let mut f = File::open(path).unwrap();
        // let mut buf = vec!();
        // f.read_to_end(&mut buf).unwrap();

        ImageReader::open(path)?.decode().map_err(|err| {
            error!("Error decoding image: {}", err);
            io::Error::new(io::ErrorKind::InvalidData, err)
        })

    }

    // todo: handle errors
    pub fn save_image(filename: &str, buffer: String, ext: String) -> Result<(), String> {
        let v = STANDARD.decode(buffer).map_err(|err| err.to_string())?;

        let mut file = Self::new_rw_file(Path::new(&Self::get_filepath(filename, &ext)))
            .map_err(|err| err.to_string())?;

        file.write(&v).map_err(|err| err.to_string())?;

        Ok(())
    }

    fn new_rw_file(path: &Path) -> Result<File, io::Error> {
        File::options().read(true).write(true).create_new(true).open(path)
    }

    fn get_filepath(filename: &str, ext: &str) -> String {
        format!("./{}/static/{}.{}", BASE_DIR, filename, ext)
    }
}