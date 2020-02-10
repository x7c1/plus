use crate::verbs::AsBytes;
use std::fmt;

pub struct SecretKey(String);

impl SecretKey {
    pub fn new<A: Into<String>>(value: A) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsBytes for SecretKey {
    fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl fmt::Debug for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let visible_length = 4;
        let masked = {
            let start = self.0.len() - visible_length;
            format!("{:*>20}", &self.0[start..])
        };
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
}
