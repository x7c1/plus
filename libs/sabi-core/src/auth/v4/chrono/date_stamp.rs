#[derive(Debug)]
pub struct DateStamp(String);

impl DateStamp {
    pub fn new<A: Into<String>>(key: A) -> DateStamp {
        DateStamp(key.into())
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl<'a> Into<&'a [u8]> for &'a DateStamp {
    fn into(self) -> &'a [u8] {
        self.0.as_bytes()
    }
}
