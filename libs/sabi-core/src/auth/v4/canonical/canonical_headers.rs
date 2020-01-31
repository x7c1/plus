use http::HeaderMap;

#[derive(Debug)]
pub struct CanonicalHeaders(String);

impl CanonicalHeaders {
    pub fn new<A: Into<String>>(value: A) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<&HeaderMap> for CanonicalHeaders {
    fn from(headers: &HeaderMap) -> Self {
        /*
            rf.
            https://github.com/durch/rust-s3/blob/a2b6879f3be920d61394d2c5ebecdfc779d20713/src/signing.rs#L53
        */
        let mut keyvalues = headers
            .iter()
            .filter_map(|(key, value)| {
                // Values that are not strings are silently dropped
                // (AWS wouldn't accept them anyway)
                if let Ok(value_inner) = value.to_str() {
                    Some(key.as_str().to_lowercase() + ":" + value_inner.trim())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>();

        keyvalues.sort();
        CanonicalHeaders::new(keyvalues.join("\n"))
    }
}
