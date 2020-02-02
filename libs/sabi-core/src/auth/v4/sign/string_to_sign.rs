use crate::auth::v4::canonical::CanonicalRequest;
use crate::auth::v4::chrono::AmzDate;
use crate::auth::v4::sign::algorithm::Algorithm;
use crate::auth::v4::sign::CredentialScope;

#[derive(Debug)]
pub struct StringToSign<'a> {
    algorithm: &'a Algorithm,
    date: AmzDate,
    scope: &'a CredentialScope,
    canonical_request: &'a CanonicalRequest,
}
