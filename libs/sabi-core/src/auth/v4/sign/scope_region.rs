#[derive(Debug)]
pub struct ScopeRegion(String);

impl ScopeRegion {
    pub fn new<A: Into<String>>(key: A) -> Self {
        Self(key.into())
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl<'a> Into<&'a [u8]> for &'a ScopeRegion {
    fn into(self) -> &'a [u8] {
        self.0.as_bytes()
    }
}
