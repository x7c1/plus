use http::HeaderMap;

#[derive(Debug)]
pub struct SignedHeaders(String);

impl SignedHeaders {
    pub fn new<A: Into<String>>(value: A) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<&HeaderMap> for SignedHeaders {
    fn from(headers: &HeaderMap) -> Self {
        let mut keys = headers
            .keys()
            .map(|key| key.as_str().to_lowercase())
            .collect::<Vec<String>>();

        keys.sort();
        SignedHeaders::new(keys.join(";"))
    }
}
