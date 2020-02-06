use crate::verbs::AsBytes;

#[derive(Debug)]
pub enum ScopeTermination {
    Aws4Request,
    Unknown(String),
}

impl ScopeTermination {
    pub fn new<A: Into<String>>(key: A) -> Self {
        Self::Unknown(key.into())
    }

    pub fn as_str(&self) -> &str {
        match self {
            ScopeTermination::Aws4Request => "aws4_request",
            ScopeTermination::Unknown(x) => &x,
        }
    }
}

impl AsBytes for ScopeTermination {
    fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}
