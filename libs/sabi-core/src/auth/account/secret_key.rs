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

impl<'a> Into<&'a [u8]> for &'a SecretKey {
    fn into(self) -> &'a [u8] {
        self.as_str().as_bytes()
    }
}
