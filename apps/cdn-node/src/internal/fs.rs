use tokio::{fs::File, io::{self, AsyncWriteExt, AsyncReadExt}};

const BASE_DIR: &str = ".cdn";
pub struct CDNFs;

impl CDNFs {
    pub async fn read_image(filename: &str, file_ext: &str, image_size: Option<String>) -> io::Result<Vec<u8>> {
        let path = Self::get_filepath(filename, file_ext, image_size);
        let mut f = File::options().read(true).open(path.clone()).await?;

        let mut buffer = vec!();
        f.read_to_end(&mut buffer).await?;

        Ok(buffer)
    }

    pub async fn save_image(filename: &str, buffer: Vec<u8>, ext: &str, size: Option<String>) -> io::Result<()> {
        let mut file = Self::new_rw_file(Self::get_filepath(filename, ext, size)).await?;
        file.write(&buffer).await?;

        Ok(())
    }

    async fn new_rw_file(path: String) -> io::Result<File> {
        File::options().read(true).write(true).create_new(true).open(path).await
    }

    fn get_filepath(filename: &str, ext: &str, size: Option<String>) -> String {
        let mut s = format!("./{}/static/{}", BASE_DIR, filename);
        if let Some(size) = size {
            s = format!("{}-{}", s, size);
        }

        format!("{s}.{ext}")
    }
}