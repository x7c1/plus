#[derive(Debug)]
pub struct ContentType(String);

impl ContentType {
    pub fn new<A: Into<String>>(key: A) -> Self {
        Self(key.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
