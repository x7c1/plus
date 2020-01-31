use url::Url;

#[derive(Debug)]
pub struct CanonicalUri(String);

impl CanonicalUri {
    pub fn new<A: Into<String>>(value: A) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<&Url> for CanonicalUri {
    fn from(url: &Url) -> Self {
        CanonicalUri::new(url.path())
    }
}
