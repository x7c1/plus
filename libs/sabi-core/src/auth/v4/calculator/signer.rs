use crate::auth::v4::calculator::hmac_factory::HmacFactory;
use crate::auth::v4::calculator::SigningKey;
use crate::auth::v4::sign::{CredentialScope, StringToSign};
use crate::auth::SecretKey;

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
        let raw = self.key.hmac(&value).to_hex();
        Signature(raw)
    }
}

#[derive(Debug)]
pub struct Signature(String);

impl Signature {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
