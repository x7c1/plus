#[derive(Debug)]
pub enum ScopeTermination {
    Aws4Request,
    Unknown(String),
}

impl ScopeTermination {
    pub fn as_str(&self) -> &str {
        match self {
            ScopeTermination::Aws4Request => "aws4_request",
            ScopeTermination::Unknown(x) => &x,
        }
    }
}

impl<'a> Into<&'a [u8]> for &'a ScopeTermination {
    fn into(self) -> &'a [u8] {
        self.as_str().as_bytes()
    }
}
