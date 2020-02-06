use crate::auth::account::AccessKey;
use crate::auth::v4::calculator::Signature;
use crate::auth::v4::canonical::SignedHeaders;
use crate::auth::v4::sign::{Algorithm, CredentialScope};

pub struct Authorization(String);

impl Authorization {
    pub fn new(
        algorithm: &Algorithm,
        access_key: &AccessKey,
        scope: &CredentialScope,
        signed_header: &SignedHeaders,
        signature: &Signature,
    ) -> Authorization {
        Authorization(format!(
            "{} Credential={}/{},SignedHeaders={},Signature={}",
            algorithm.as_str(),
            access_key.as_str(),
            scope.as_str(),
            signed_header.as_str(),
            signature.as_str(),
        ))
    }
}
