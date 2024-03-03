use base64::prelude::*;
use base64::engine::general_purpose::STANDARD;
use std::{fs::File, io::{self, Write}, path::Path};
use image::DynamicImage;
use image::io::Reader as ImageReader;
use logger::error;

const BASE_DIR: &str = ".cdn";
pub struct CDNFs;

impl CDNFs {
    pub fn read_image(filename: &str, file_ext: &str) -> io::Result<DynamicImage> {
        let path = Self::get_filepath(filename, file_ext);

        ImageReader::open(path)?.decode().map_err(|err| {
            error!("Error decoding image (from fs): {}", err);
            io::Error::new(io::ErrorKind::InvalidData, err)
        })

    }

    pub fn save_image(filename: &str, buffer: String, ext: String) -> io::Result<()> {
        let v = STANDARD.decode(buffer).map_err(|err| {
            error!("Error decoding image (from base64): {}", err);
            io::Error::new(io::ErrorKind::InvalidData, err)
        })?;

        let mut file = Self::new_rw_file(Path::new(&Self::get_filepath(filename, &ext)))?;
        file.write(&v)?;

        Ok(())
    }

    fn new_rw_file(path: &Path) -> io::Result<File> {
        File::options().read(true).write(true).create_new(true).open(path)
    }

    fn get_filepath(filename: &str, ext: &str) -> String {
        format!("./{}/static/{}.{}", BASE_DIR, filename, ext)
    }
}