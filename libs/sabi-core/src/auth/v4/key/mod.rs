/*
    see also:
    https://docs.aws.amazon.com/general/latest/gr/sigv4-calculate-signature.html
*/

mod hmac_factory;
pub use hmac_factory::HmacFactory;

mod signing_key;
pub use signing_key::SigningKey;

#[cfg(test)]
mod tests {
    use super::SigningKey;
    use crate::auth::v4::chrono::DateStamp;
    use crate::auth::v4::sign::{CredentialScope, ScopeTermination};
    use crate::auth::SecretKey;
    use crate::index::{RegionCode, ServiceCode};
    use crate::SabiResult;

    #[test]
    fn it_works() -> SabiResult<()> {
        let secret_key = SecretKey::new("wJalrXUtnFEMI/K7MDENG+bPxRfiCYEXAMPLEKEY");
        let scope = CredentialScope::from(
            DateStamp::new("20150830"),
            RegionCode::new("us-east-1"),
            ServiceCode::new("iam"),
            ScopeTermination::new("aws4_request"),
        );
        let signing_key = SigningKey::new(&secret_key, &scope);
        assert_eq!(
            signing_key.to_hex(),
            "c4afb1cc5771d871763a393e44b703571b55cc28424d1a5e86da6ed3c154a4b9"
        );
        Ok({})
    }
}
