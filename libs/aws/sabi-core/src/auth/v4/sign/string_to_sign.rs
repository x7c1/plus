use crate::auth::v4::canonical::CanonicalRequest;
use crate::auth::v4::chrono::AmzDate;
use crate::auth::v4::sign::algorithm::Algorithm;
use crate::auth::v4::sign::CredentialScope;
use crate::verbs::AsBytes;

#[derive(Debug)]
pub struct StringToSign(String);

impl StringToSign {
    pub fn from(
        algorithm: &Algorithm,
        date: &AmzDate,
        scope: &CredentialScope,
        request: &CanonicalRequest,
    ) -> Self {
        StringToSign(format!(
            "{algorithm}\n{amz_date}\n{scope}\n{hash}",
            algorithm = algorithm.as_str(),
            amz_date = date.as_str(),
            scope = scope.as_str(),
            hash = request.as_hash(),
        ))
    }

    pub fn new<A: Into<String>>(key: A) -> Self {
        Self(key.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl AsBytes for StringToSign {
    fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}
