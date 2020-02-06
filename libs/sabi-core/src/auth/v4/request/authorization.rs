use crate::auth::v4::calculator::Signature;
use crate::auth::v4::canonical::SignedHeaders;
use crate::auth::v4::request::Credential;
use crate::auth::v4::sign::Algorithm;

pub struct Authorization(String);

impl Authorization {
    pub fn new(
        algorithm: &Algorithm,
        credential: &Credential,
        signed_header: &SignedHeaders,
        signature: &Signature,
    ) -> Authorization {
        Authorization(format!(
            "{} Credential={},SignedHeaders={},Signature={}",
            algorithm.as_str(),
            credential.as_str(),
            signed_header.as_str(),
            signature.as_str(),
        ))
    }
}
