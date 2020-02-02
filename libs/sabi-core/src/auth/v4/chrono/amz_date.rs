#[derive(Debug)]
pub struct AmzDate(String);

impl AmzDate {
    pub fn new<A: Into<String>>(value: A) -> AmzDate {
        AmzDate(value.into())
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
