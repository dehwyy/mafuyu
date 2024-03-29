use std::str::FromStr;

pub struct Cookie {
    pub key: String,
    pub value: String,
    pub http_only: bool,
    pub max_age: Option<i32>
}

impl Default for Cookie {
    fn default() -> Self {
        Self {
            key: "".to_string(),
            value: "".to_string(),
            http_only: false,
            max_age: None
        }
    }
}

impl Cookie {
    pub fn header_key() -> http::HeaderName {
        http::HeaderName::from_str("Set-Cookie").expect("cannot create `Set-Cookie` key from `str`")
    }
    pub fn new(&self) -> http::HeaderValue {
        let mut s = format!("{}={}; path=/; Secure; SameSite=None", self.key, self.value);
        if self.http_only {
            s = format!("{}; HttpOnly", s);
        }

        if let Some(max_age) = self.max_age {
            s = format!("{}; Max-Age={}", s, max_age);
        }


        http::HeaderValue::from_str(&s).expect("cannot create `Set-Cookie` value from `str`")
    }
}

pub struct Headers;

impl Headers {
    pub fn extract_header(headers: &http::HeaderMap, key: &str) -> Option<String> {
        if let Some(Some(v)) = headers.get(key).map(|v| {
            let s = v.to_str();
            match s {
                Ok(s) => {
                    match s.len() {
                        0 => None,
                        _ => Some(s.to_string())
                    }
                },
                Err(_) => return None
            }
        }) {
            return Some(v)
        };

        return None
    }
}