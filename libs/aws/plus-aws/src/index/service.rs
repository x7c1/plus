use crate::verbs::AsBytes;

/// see also:
///  * [AWS Service Endpoints - AWS General Reference](https://docs.aws.amazon.com/general/latest/gr/rande.html)
///
#[derive(Debug)]
pub enum ServiceCode {
    Iam,
    S3,
    Any(String),
}

impl ServiceCode {
    pub fn any<A: Into<String>>(key: A) -> Self {
        Self::Any(key.into())
    }

    pub fn as_str(&self) -> &str {
        match self {
            ServiceCode::Iam => "iam",
            ServiceCode::S3 => "s3",
            ServiceCode::Any(code) => &code,
        }
    }
}

impl AsBytes for ServiceCode {
    fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}
