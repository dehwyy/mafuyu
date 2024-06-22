use std::cmp::{max, min};
use std::io;
use ril::{ImageFormat, Pixel, Rgba};
use mafuyu_lib::image::{core::Base64ImageType, Image as ImageOperations};
use crate::internal::fs::CDNFs;

const MD: u32 = 500;
const SM: u32 = 300;

type RilImage = ril::Image<Rgba>;
type RilAnimatedImage = ril::ImageSequence<Rgba>;


enum ImageTarget {
    Webp(RilImage),
    Gif(RilAnimatedImage),
}

impl Into<ImageTarget> for RilImage {
    fn into(self) -> ImageTarget {
        ImageTarget::Webp(self)
    }
}

impl Into<ImageTarget> for RilAnimatedImage {
    fn into(self) -> ImageTarget {
        ImageTarget::Gif(self)
    }
}


pub struct Image;

impl Image {
    pub async fn save_image(filename: String, image: String, image_ext: String) -> ril::Result<()> {
        let image_buf = ImageOperations::from_base64_to_bytes(&image)?;

        let ext = Base64ImageType::from(image_ext);
        match ext {
            Base64ImageType::Webp => {
                let image: RilImage  = ril::Image::from_bytes(ImageFormat::WebP, image_buf.as_slice())?;
                let images = Self::resize_image_default_sizes(image).await;

                Self::save_resized(filename, images).await?;
            },
            Base64ImageType::Gif => {
                CDNFs::save_image(&filename, image_buf, "gif", None).await?;
                // TODO: consider.
                // Well, I don't exactly know whether `gif` image should be resized or not.
                // I may put just **max_size* limit
                // But... I wrote these abstractions for `gif` + other formats...

                // let image_frames = ril::ImageSequence::<Rgba>::from_bytes(ImageFormat::Gif, image_buf.as_slice())?;
                //
                // let mut outputs: [ril::ImageSequence<Rgba>; 3] = [ril::ImageSequence::<Rgba>::new(), ril::ImageSequence::<Rgba>::new(), ril::ImageSequence::<Rgba>::new()];
                //
                // for frame in image_frames {
                //     let frame: Frame<Rgba> = frame?;
                //
                //     Self::resize_image_default_sizes(frame.into_image()).await.into_iter().enumerate().for_each(|(i, image)| {
                //         outputs[i].push_frame(Frame::from_image(image));
                //     });
                // }
                //
                // Self::save_resized(filename, outputs).await?;
            }
        };

        Ok(())
    }

    async fn resize_image_default_sizes(image: RilImage) -> [RilImage; 3] {
        let mut md_image = image.clone();
        let md = Self::resize_image_like(&mut md_image, (MD, MD));

        let mut sm_image = image.clone();
        let sm = Self::resize_image_like(&mut sm_image, (SM, SM));

        let _ = tokio::join!(md, sm);

        [image, md_image, sm_image]
    }

    /// - `max_size` (w, h).
    /// Would be aligned to smaller value with saved proportions <br />
    /// *Example*: (1920, 1080) & (500, 500) -> (500, 281,25), `k` = 3.84 (where `k` is the ratio)
    /// - `size` (sm | md)
    async fn resize_image_like(image: &mut RilImage, max_size: (u32, u32)) {
        let initial_size = (image.width(), image.height());

        let pivot = max(initial_size.0, initial_size.1) as f64;
        let k  = pivot / (min(max_size.0, max_size.1) as f64) ;

        if k > 1.0 {
            let size = ((initial_size.0 as f64) / k, (initial_size.1 as f64) / k);
            image.resize(size.0 as u32, size.1 as u32, ril::ResizeAlgorithm::Lanczos3);
        }
    }

    async fn save_resized<T>(filename: String, images: [T; 3]) -> io::Result<()>
    where T: Into<ImageTarget> + Clone {
        let standard = Self::save(&filename, images[0].clone(), None);
        let medium = Self::save(&filename, images[1].clone(), Some("md".to_string()));
        let small = Self::save(&filename, images[2].clone(), Some("sm".to_string()));

        let (standard, medium, small) = tokio::join!(standard, medium, small);
        (standard?, medium?, small?);

        Ok(())
    }

    async fn save<T>(filename: &str, image: T, size: Option<String>) -> io::Result<()>
    where T: Into<ImageTarget> + Clone {
        let mut buffer = Vec::new();
        let (r, ext)  = match image.into() {
            ImageTarget::Gif(image) => (image.encode(ImageFormat::Gif, &mut buffer), "gif".to_string()),
            ImageTarget::Webp(image) => (image.encode(ImageFormat::WebP, &mut buffer), "webp".to_string())
        };

        r.map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

        CDNFs::save_image(filename, buffer, &ext, size).await?;

        Ok(())
    }
}