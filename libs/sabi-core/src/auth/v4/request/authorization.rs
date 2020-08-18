use crate::auth::account::AccessKey;
use crate::auth::v4::calculator::Signature;
use crate::auth::v4::canonical::SignedHeaders;
use crate::auth::v4::sign::{Algorithm, CredentialScope};
use crate::PlusResult;
use http::HeaderValue;

pub struct Authorization(String);

impl Authorization {
    pub fn new(
        algorithm: &Algorithm,
        access_key: &AccessKey,
        scope: &CredentialScope,
        signed_headers: &SignedHeaders,
        signature: &Signature,
    ) -> Authorization {
        Authorization(format!(
            "{algorithm} Credential={access_key}/{scope},SignedHeaders={header},Signature={signature}",
            algorithm = algorithm.as_str(),
            access_key = access_key.as_str(),
            scope = scope.as_str(),
            header = signed_headers.as_str(),
            signature = signature.as_str(),
        ))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn to_header_value(&self) -> PlusResult<HeaderValue> {
        let value = self.as_str().parse()?;
        Ok(value)
    }
}
