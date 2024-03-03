pub enum Base64ImageType {
    Webp,
    Gif,
}

impl From<String> for Base64ImageType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "webp" => Self::Webp,
            "gif" => Self::Gif,
            _ => Self::Webp
        }
    }
}

impl std::fmt::Display for Base64ImageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Webp => "webp",
            Self::Gif => "gif",
        };

        write!(f, "{}", s)
    }
}
