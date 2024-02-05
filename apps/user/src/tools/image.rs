pub enum ImageType {
    Url(String),
    Base64(String)
}

pub struct Image;

impl Image {
    pub fn parse(image: &String) -> Option<ImageType> {
        match image.starts_with("http") {
            // Already an url, use it as picture value.
            true => Some(ImageType::Url(image.to_string())),
            // Otherwise, upload image.
            // `Image` is most likely `dataUrl`, which starts like `data:image/png;base64, (random_base64_text)`.
            // We should remove invalid base64 url at the start by splitting_once and taking 2nd part.
            false => image.split_once(",")
                .map(|parts| ImageType::Base64(parts.1.to_string()))
        }
    }
}