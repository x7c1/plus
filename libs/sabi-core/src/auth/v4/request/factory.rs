use crate::auth::account::Credentials;
use crate::auth::v4::calculator::Signer;
use crate::auth::v4::canonical::{CanonicalRequest, HashedPayload};
use crate::auth::v4::chrono::{AmzDate, DateStamp};
use crate::auth::v4::request::Authorization;
use crate::auth::v4::sign::{Algorithm, CredentialScope, ScopeTermination, StringToSign};
use crate::index::{RegionCode, ServiceCode};
use chrono::{DateTime, Utc};
use http::{HeaderMap, Method};
use url::Url;

pub struct AuthorizationFactory<'a> {
    pub credentials: &'a Credentials,
    pub fragment: &'a AuthorizationFragment,
    pub region_code: RegionCode,
    pub service_code: ServiceCode,
}

impl AuthorizationFactory<'_> {
    pub fn create(self, headers: &HeaderMap) -> Authorization {
        let scope = CredentialScope::from(
            DateStamp::from(&self.fragment.requested_at),
            self.region_code,
            self.service_code,
            ScopeTermination::Aws4Request,
        );
        let request = CanonicalRequest::from(
            &self.fragment.method,
            &self.fragment.url,
            &headers,
            &self.fragment.hashed_payload,
        );
        let algorithm = Algorithm::HmacSha256;
        let string_to_sign = {
            StringToSign::from(
                &algorithm,
                &AmzDate::from(&self.fragment.requested_at),
                &scope,
                &request,
            )
        };
        let signature = {
            let signer = Signer::new(&self.credentials.secret_key, &scope);
            signer.sign(string_to_sign)
        };
        Authorization::new(
            &algorithm,
            &self.credentials.access_key,
            &scope,
            &request.signed_headers,
            &signature,
        )
    }
}

pub struct AuthorizationFragment {
    pub url: Url,
    pub method: Method,
    pub hashed_payload: HashedPayload,
    pub requested_at: DateTime<Utc>,
}
