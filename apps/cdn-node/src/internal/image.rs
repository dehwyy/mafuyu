use std::io::Cursor;
use image::{ImageFormat, ImageResult};
use image::imageops::{resize, Triangle as TriangleFilter};

pub struct PipeImagePayload {
    pub image: image::DynamicImage,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

pub struct Image;

impl Image {
    pub fn pipe_image(p: PipeImagePayload) -> ImageResult<Vec<u8>> {
        let img = {
            let nheight = p.height;
            let nwidth = p.width;

            match (nheight, nwidth) {
                (Some(w), Some(h)) => {
                    resize(&p.image, w, h, TriangleFilter)
                },
                _ => p.image.into()
            }

        };


        let mut v = vec!();
        img.write_to(&mut  Cursor::new(&mut v), ImageFormat::Jpeg)?;

        Ok(v)
    }
}