pub struct Hasher;

const HASH_SALT: u32 = 10;

impl Hasher {
  pub fn hash<S: AsRef<[u8]>>(s: S) -> Result<String, ()> {
    Ok(bcrypt::hash(s, HASH_SALT).map_err(|err| {
      eprintln!("Error hashing password: {}", err);
    })?)
  }

  pub fn verify<S: AsRef<[u8]>>(s: S, hash: &str) -> Result<bool, ()> {
    Ok(bcrypt::verify(s, hash).map_err(|err| {
      eprintln!("Error verifying password: {}", err);
    })?)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_hasher() {
    let s = "some_string_9489";
    let hash = Hasher::hash(s).unwrap();
    assert_eq!(true, hash.len() > 1);
    assert_eq!(true, Hasher::verify(s, &hash).unwrap());
  }
}
