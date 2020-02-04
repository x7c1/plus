use crate::auth::v4::key::hmac_factory::CanGenerateHmac;
use crate::auth::v4::key::SigningKey;
use crate::auth::v4::sign::{CredentialScope, StringToSign};
use crate::auth::SecretKey;
use hex::ToHex;
use hmac::Mac;

#[derive(Debug)]
pub struct Signer {
    pub key: SigningKey,
}

impl Signer {
    pub fn new(secret_key: &SecretKey, scope: &CredentialScope) -> Signer {
        let key = SigningKey::new(secret_key, scope);
        Signer { key }
    }

    pub fn sign(&self, value: StringToSign) -> Signature {
        let key = self.key.as_bytes();
        let mut hmac = key.to_hmac();
        hmac.input(value.as_str().as_bytes());

        let value = hmac.result().code().encode_hex();
        Signature(value)
    }
}

#[derive(Debug)]
pub struct Signature(String);

impl Signature {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
