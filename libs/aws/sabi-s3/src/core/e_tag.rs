#[derive(Debug)]
pub struct ETag(String);

impl ETag {
    pub fn new<A: Into<String>>(name: A) -> Self {
        Self(name.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
