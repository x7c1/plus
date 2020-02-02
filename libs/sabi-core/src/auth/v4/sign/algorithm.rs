#[derive(Debug)]
pub enum Algorithm {
    HmacSha256,
}

impl Algorithm {
    pub fn as_str(&self) -> &str {
        match self {
            Algorithm::HmacSha256 => "AWS4-HMAC-SHA256",
        }
    }
}
