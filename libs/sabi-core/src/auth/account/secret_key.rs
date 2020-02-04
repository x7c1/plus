use crate::verbs::AsBytes;

#[derive(Debug)]
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
