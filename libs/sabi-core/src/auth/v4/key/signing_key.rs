use crate::auth::v4::key::HmacFactory;
use crate::auth::v4::sign::CredentialScope;
use crate::auth::SecretKey;
use crate::verbs::AsBytes;
use hex::ToHex;

#[derive(Debug)]
pub struct SigningKey(Vec<u8>);

/// https://docs.aws.amazon.com/general/latest/gr/sigv4-create-string-to-sign.html
impl SigningKey {
    pub fn new(secret_key: &SecretKey, scope: &CredentialScope) -> SigningKey {
        let hmac = ("AWS4", secret_key)
            .hmac(&scope.date)
            .hmac(&scope.region)
            .hmac(&scope.service)
            .hmac(&scope.termination);

        SigningKey(hmac.code)
    }

    pub fn to_hex(&self) -> String {
        self.0.encode_hex()
    }
}

impl AsBytes for SigningKey {
    fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}
