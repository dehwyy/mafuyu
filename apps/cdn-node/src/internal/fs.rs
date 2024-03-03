use std::{fs::File, io::{self, Write}, path::Path};
use std::io::Read;

const BASE_DIR: &str = ".cdn";
pub struct CDNFs;

impl CDNFs {
    pub fn read_image(filename: &str, file_ext: &str, image_size: Option<String>) -> io::Result<Vec<u8>> {
        let path = Self::get_filepath(filename, file_ext, image_size);

        let mut f = File::options().read(true).open(path.clone())?;

        let mut buffer = vec!();
        f.read_to_end(&mut buffer)?;

        Ok(buffer)

        // let mut i: ril::Image<ril::Rgba> = ril::Image::open(path).unwrap();
        // let mut buffer = Vec::new();
        // i.encode(ril::ImageFormat::WebP, &mut buffer).unwrap();
    }

    pub fn save_image(filename: &str, buffer: Vec<u8>, ext: &str, size: Option<String>) -> io::Result<()> {
        let mut file = Self::new_rw_file(Path::new(&Self::get_filepath(filename, ext, size)))?;
        file.write(&buffer)?;

        Ok(())
    }

    fn new_rw_file(path: &Path) -> io::Result<File> {
        File::options().read(true).write(true).create_new(true).open(path)
    }

    fn get_filepath(filename: &str, ext: &str, size: Option<String>) -> String {
        let mut s = format!("./{}/static/{}", BASE_DIR, filename);
        if let Some(size) = size {
            s = format!("{}-{}", s, size);
        }

        format!("{s}.{ext}")
    }
}