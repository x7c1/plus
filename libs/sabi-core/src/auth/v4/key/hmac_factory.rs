use crate::auth::SecretKey;
use hmac::{Hmac, Mac};
use sha2::Sha256;

pub trait HmacFactory: CanGenerateHmac {
    fn hmac<'a, A>(&'a self, target: A) -> OutputHmac
    where
        A: Into<&'a [u8]>,
    {
        let mut hmac: Hmac<Sha256> = self.generate();
        hmac.input(target.into());
        OutputHmac {
            code: hmac.result().code().to_vec(),
        }
    }
}

pub trait CanGenerateHmac {
    fn generate(&self) -> Hmac<Sha256>;
}

impl HmacFactory for (&str, &SecretKey) {}

impl CanGenerateHmac for (&str, &SecretKey) {
    fn generate(&self) -> Hmac<Sha256> {
        let (first, second) = self;
        let key = format!("{}{}", first, second.as_str());
        // todo: avoid unwrap
        Hmac::new_varkey(key.as_bytes()).unwrap()
    }
}

pub struct OutputHmac {
    code: Vec<u8>,
}

impl OutputHmac {
    pub fn code(self) -> Vec<u8> {
        self.code
    }
}

impl HmacFactory for OutputHmac {}

impl CanGenerateHmac for OutputHmac {
    fn generate(&self) -> Hmac<Sha256> {
        let key: &[u8] = &self.code;
        // todo: avoid unwrap
        Hmac::new_varkey(key).unwrap()
    }
}
