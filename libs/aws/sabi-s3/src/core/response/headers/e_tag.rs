use crate::core;
use crate::core::response::headers::AwsHeader;

#[derive(Debug)]
pub struct ETag(String);

impl ETag {
    pub fn new<A: Into<String>>(name: A) -> Self {
        Self(name.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl<'a> AwsHeader<'a> for ETag {
    const HEADER_NAME: &'a str = "ETag";

    fn new<A: Into<String>>(a: A) -> core::Result<Self> {
        Ok(Self::new(a))
    }
}
