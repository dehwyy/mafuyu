use base64::prelude::*;
use base64::engine::general_purpose::STANDARD;
use std::io;

pub mod core;

use crate::image::core::*;

pub enum Image {
    Url(String),
    Base64(String, Base64ImageType) // (base64, image_type (webp, gif) )
}

impl Image {
    ///
    pub fn try_parse(image: &String) -> Option<Self> {
        match image.starts_with("http") {
            // Already an url, use it as picture value.
            true => Some(Image::Url(image.to_string())),
            // Otherwise, upload image.
            // `Image` is most likely `dataUrl`, which starts like `data:image/png;base64, (random_base64_text)`.
            // We should remove invalid base64 url at the start by splitting_once and taking 2nd part.
            false => image.split_once(",")
                .map(|(headers, base64)| {
                    let data_type = headers.split_terminator(&[':', ';'][..]).nth(1).map(|data_type |{
                        match data_type {
                            "image/gif" => Base64ImageType::Gif,
                            _ => Base64ImageType::Webp
                        }
                    }).unwrap_or(Base64ImageType::Webp);

                    Self::Base64(base64.to_string(), data_type)
                })
        }
    }

    pub fn from_base64_to_bytes(image: &String) -> io::Result<Vec<u8>> {
        STANDARD.decode(image).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
    }
}