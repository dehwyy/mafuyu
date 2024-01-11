
use serde_json::Value as JsonValue;
pub use serde_json::json as Encode; // re-exported macros

// Json decoder
pub struct Decoder {
  inner: JsonValue // json-formated string, which should be parseable
}

impl Decoder {
  // s - json-formated string
  pub fn new(s: String) -> Result<Self, ()> {
    Ok(Self {
      inner: serde_json::from_str(&s).map_err(|err| {
        eprintln!("cannot parse json {err}");
      })?
    })
  }

  pub fn new_from_vec(s: Vec<u8>) -> Result<Self, ()> {
    Ok(Self {
      inner: serde_json::from_slice(s.as_slice()).map_err(|err| {
        eprintln!("cannot parse json {err}");
      })?
    })
  }

  pub fn get_bool(&self, key: &str) -> Option<bool> {
    self.inner[&key].as_bool()
  }

  pub fn get_string(&self, key: &str) -> Option<String> {
    self.inner[key].as_str().map(|s| s.to_string())
  }

  pub fn get_i64(&self, key: &str) -> Option<i64> {
    self.inner[&key].as_i64()
  }

  pub fn get_string_array(&self, key: &str) -> Option<Vec<String>> {
    self.inner[&key].as_array()
    .map(|array|
      array
      .iter()
      .map(|v| v.as_str().map(|v| v.to_string()).expect("cannot convert value to string"))
    .collect())
  }
}

pub struct Encoder {
  inner: JsonValue
}

impl Encoder {
  pub fn new(v: JsonValue) -> Self {
    Self {
      inner: v
    }
  }

  pub fn string(&self) -> String {
    self.inner.to_string()
  }
}
