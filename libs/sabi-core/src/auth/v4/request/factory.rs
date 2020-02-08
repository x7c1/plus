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
    pub fragment: &'a CanonicalFragment,
    pub scope: CredentialScope,
    pub requested_at: &'a DateTime<Utc>,
}

impl AuthorizationFactory<'_> {
    pub fn new<'a>(
        credentials: &'a Credentials,
        fragment: &'a CanonicalFragment,
        region_code: RegionCode,
        service_code: ServiceCode,
        requested_at: &'a DateTime<Utc>,
    ) -> AuthorizationFactory<'a> {
        let scope = CredentialScope::from(
            DateStamp::from(&requested_at),
            region_code,
            service_code,
            ScopeTermination::Aws4Request,
        );
        AuthorizationFactory {
            credentials,
            fragment,
            scope,
            requested_at,
        }
    }

    pub fn create_from(self, headers: &HeaderMap) -> Authorization {
        let request = self.fragment.to_canonical(headers);
        let algorithm = Algorithm::HmacSha256;
        let string_to_sign = StringToSign::from(
            &algorithm,
            &AmzDate::from(&self.requested_at),
            &self.scope,
            &request,
        );
        let signature = {
            let signer = Signer::new(&self.credentials.secret_key, &self.scope);
            signer.sign(string_to_sign)
        };
        Authorization::new(
            &algorithm,
            &self.credentials.access_key,
            &self.scope,
            &request.signed_headers,
            &signature,
        )
    }
}

pub struct CanonicalFragment {
    pub url: Url,
    pub method: Method,
    pub hashed_payload: HashedPayload,
}

impl CanonicalFragment {
    pub fn to_canonical(&self, headers: &HeaderMap) -> CanonicalRequest {
        CanonicalRequest::from(&self.method, &self.url, &headers, &self.hashed_payload)
    }
}
