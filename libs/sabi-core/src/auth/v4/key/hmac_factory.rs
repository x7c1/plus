use crate::auth::SecretKey;
use hex::ToHex;
use hmac::{Hmac, Mac};
use sha2::Sha256;

pub trait HmacFactory: CanGenerateHmac {
    fn hmac<'a, A>(&'a self, target: A) -> OutputHmac
    where
        A: Into<&'a [u8]>,
    {
        let mut hmac: Hmac<Sha256> = self.to_hmac();
        hmac.input(target.into());
        OutputHmac {
            code: hmac.result().code().to_vec(),
        }
    }
}

impl HmacFactory for [u8] {}

impl HmacFactory for (&str, &SecretKey) {}

pub trait CanGenerateHmac {
    fn to_hmac(&self) -> Hmac<Sha256>;
}

impl CanGenerateHmac for [u8] {
    fn to_hmac(&self) -> Hmac<Sha256> {
        Hmac::new_varkey(self).expect("HMAC can take key of any size")
    }
}

impl CanGenerateHmac for (&str, &SecretKey) {
    fn to_hmac(&self) -> Hmac<Sha256> {
        let (first, second) = self;
        let key = format!("{}{}", first, second.as_str());
        key.as_bytes().to_hmac()
    }
}

pub struct OutputHmac {
    code: Vec<u8>,
}

impl OutputHmac {
    pub fn code(self) -> Vec<u8> {
        self.code
    }
    pub fn to_hex(&self) -> String {
        self.code.encode_hex()
    }
}

impl HmacFactory for OutputHmac {}

impl CanGenerateHmac for OutputHmac {
    fn to_hmac(&self) -> Hmac<Sha256> {
        self.code.to_hmac()
    }
}
