#[derive(Debug)]
pub struct AccessKey(String);

impl AccessKey {
    pub fn new<A: Into<String>>(value: A) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
