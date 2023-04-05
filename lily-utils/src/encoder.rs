use argon2::{self, Config};
use anyhow::Result;

static SALT: &[u8; 11] = b"sankar_boro";

fn encode(u: &[u8]) -> Result<String> {
  let config = Config::default();
  Ok(argon2::hash_encoded(u, SALT, &config)?)
}

pub fn encrypt_text(user_password: &str) -> Result<String> {
  let b = user_password.as_bytes();
  encode(b)
}

pub fn encrypt_text_bytes(user_password: &Vec<u8>) -> Result<String> {
  encode(user_password)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_password() {
      // $argon2i$v=19$m=4096,t=3,p=1$c2Fua2FyX2Jvcm8$rNL8kMFnoZKT82Qz6ZrwkXBidwXCkPVgA0DH+1f9CKw
      let x = encrypt_text("sankar").unwrap();
      assert_eq!(x, "$argon2i$v=19$m=4096,t=3,p=1$c2Fua2FyX2Jvcm8$rNL8kMFnoZKT82Qz6ZrwkXBidwXCkPVgA0DH+1f9CKw");
    }
}