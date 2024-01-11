use regex::Regex;

// Credentials validator
pub struct Validator;

type ValidationResult = Result<(), String>;

impl Validator {
  /// validates username
  pub fn username(username: &str) -> ValidationResult {
    // perform some validation
    // for now just 1:
    if username.len() < 1 {
      return Err("username too short".to_string());
    };  // xd
    Ok(())
  }

  pub fn email(email: &str) -> ValidationResult {
    let reg = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").expect("Invalid regex");

    if !reg.is_match(email) {
      return Err("Invalid email".to_string());
    }

    Ok(())
  }

  pub fn password(password: &str) -> ValidationResult {
    if password.len() < 8 {
      return Err("password too short".to_string());
    }

    Ok(())
  }
}
