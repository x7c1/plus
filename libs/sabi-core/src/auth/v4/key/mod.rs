/*
    see also:
    https://docs.aws.amazon.com/general/latest/gr/sigv4-calculate-signature.html
*/

mod hmac_factory;
pub use hmac_factory::{CanGenerateHmac, HmacFactory};

mod signer;
pub use signer::{Signature, Signer};

mod signing_key;
pub use signing_key::SigningKey;

#[cfg(test)]
mod tests {
    use super::{Signer, SigningKey};
    use crate::auth::v4::canonical::{CanonicalRequest, HashedPayload};
    use crate::auth::v4::chrono::{AmzDate, DateStamp};
    use crate::auth::v4::sign::{Algorithm, CredentialScope, ScopeTermination, StringToSign};
    use crate::auth::SecretKey;
    use crate::http::Headers;
    use crate::index::{RegionCode, ServiceCode};
    use crate::SabiResult;
    use http::{HeaderMap, Method};
    use url::Url;

    #[test]
    fn it_can_create_signing_key() -> SabiResult<()> {
        let secret_key = create_secret_key();
        let scope = create_scope();
        let signing_key = SigningKey::new(&secret_key, &scope);

        assert_eq!(
            signing_key.to_hex(),
            "c4afb1cc5771d871763a393e44b703571b55cc28424d1a5e86da6ed3c154a4b9"
        );
        Ok({})
    }

    #[test]
    fn it_can_calculate_signature() -> SabiResult<()> {
        let scope = create_scope();
        let string_to_sign = {
            let link = "https://iam.amazonaws.com/?Action=ListUsers&Version=2010-05-08";
            let url = Url::parse(link).unwrap();

            let request = CanonicalRequest::from(
                &Method::GET,
                &url,
                &to_headers(&url)?,
                HashedPayload::empty(),
            );
            StringToSign::from(
                &Algorithm::HmacSha256,
                &AmzDate::new("20150830T123600Z"),
                &scope,
                &request,
            )
        };
        let signature = {
            let secret_key = create_secret_key();
            let signer = Signer::new(&secret_key, &scope);
            signer.sign(string_to_sign)
        };
        assert_eq!(
            signature.as_str(),
            "5d672d79c15b13162d9279b0855cfba6789a8edb4c82c400e06b5924a6f2b5d7"
        );
        Ok({})
    }
    fn create_secret_key() -> SecretKey {
        SecretKey::new("wJalrXUtnFEMI/K7MDENG+bPxRfiCYEXAMPLEKEY")
    }

    fn create_scope() -> CredentialScope {
        CredentialScope::from(
            DateStamp::new("20150830"),
            RegionCode::UsEast1,
            ServiceCode::Iam,
            ScopeTermination::Aws4Request,
        )
    }

    fn to_headers(url: &Url) -> SabiResult<HeaderMap> {
        let headers = HeaderMap::new()
            .push(("host", url.host_str().unwrap()))?
            .push((
                "content-type",
                "application/x-www-form-urlencoded; charset=utf-8",
            ))?
            .push(("x-amz-date", "20150830T123600Z"))?;

        Ok(headers)
    }
}
