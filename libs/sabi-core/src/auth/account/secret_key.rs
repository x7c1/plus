use crate::verbs::AsBytes;
use characters::{AsStr, MultiByte};
use std::fmt;

pub struct SecretKey(String);

impl SecretKey {
    pub fn new<A: Into<String>>(value: A) -> Self {
        Self(value.into())
    }
}

impl AsBytes for SecretKey {
    fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl AsStr for SecretKey {
    fn as_str(&self) -> &str {
        &self.0
    }
}

impl MultiByte for SecretKey {}

impl fmt::Debug for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let masked = format!("{:*>20}", &self.tail(4));
        f.debug_tuple("SecretKey").field(&masked).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::SecretKey;

    #[test]
    fn it_should_be_masked() {
        let key = SecretKey::new("wJalrXUtnFEMI/K7MDENG+bPxRfiCYEXAMPLEKEY");
        let masked = format!("{:?}", key);

        assert_eq!(masked, r#"SecretKey("****************EKEY")"#);
        assert_eq!(masked.len(), 20 + r#"SecretKey("")"#.len());
    }

    #[test]
    fn it_should_be_masked_even_if_multibyte_given() {
        let key = SecretKey("wJalrXUtnFEMI/K7MDENG+bPxRfiCYEXAM途中に日本語".to_string());
        let masked = format!("{:?}", key);

        assert_eq!(masked, r#"SecretKey("****************に日本語")"#);
        assert_eq!(masked.chars().count(), 20 + r#"SecretKey("")"#.len());
    }
}
