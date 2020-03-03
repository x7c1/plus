use crate::auth::account::Credentials;
use crate::auth::v4::calculator::Signer;
use crate::auth::v4::canonical::HeadersCapturer;
use crate::auth::v4::chrono::AmzDate;
use crate::auth::v4::request::Authorization;
use crate::auth::v4::sign::{Algorithm, CredentialScope, StringToSign};
use chrono::{DateTime, Utc};
use http::HeaderMap;

pub struct AuthorizationFactory<'a> {
    credentials: &'a Credentials,
    capturer: HeadersCapturer<'a>,
    scope: &'a CredentialScope<'a>,
    amz_date: AmzDate,
}

impl AuthorizationFactory<'_> {
    pub fn new<'a, A>(credentials: &'a Credentials, request: &'a A) -> AuthorizationFactory<'a>
    where
        A: AuthorizationParts,
    {
        AuthorizationFactory {
            credentials,
            capturer: request.to_capturer(),
            scope: request.to_scope(),
            amz_date: AmzDate::from(request.to_datetime()),
        }
    }

    pub fn amz_date(&self) -> &AmzDate {
        &self.amz_date
    }

    pub fn create_from(self, headers: &HeaderMap) -> Authorization {
        let request = self.capturer.capture(headers);
        let algorithm = Algorithm::HmacSha256;
        let string_to_sign = StringToSign::from(&algorithm, &self.amz_date, &self.scope, &request);
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

pub trait AuthorizationParts {
    fn to_capturer(&self) -> HeadersCapturer;
    fn to_scope(&self) -> &CredentialScope;
    fn to_datetime(&self) -> &DateTime<Utc>;
}
