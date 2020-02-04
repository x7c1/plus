use crate::auth::SecretKey;
use crate::verbs::AsBytes;
use hex::ToHex;
use hmac::{Hmac, Mac};
use sha2::Sha256;

pub trait HmacFactory: CanGenerateHmac {
    fn hmac<A>(&self, target: A) -> OutputHmac
    where
        A: AsBytes,
    {
        let mut hmac: Hmac<Sha256> = self.to_hmac();
        hmac.input(target.as_bytes());
        OutputHmac {
            code: hmac.result().code().to_vec(),
        }
    }
}

impl<A> HmacFactory for A where A: CanGenerateHmac {}

pub trait CanGenerateHmac {
    fn to_hmac(&self) -> Hmac<Sha256>;
}

impl<A> CanGenerateHmac for A
where
    A: AsBytes,
{
    fn to_hmac(&self) -> Hmac<Sha256> {
        Hmac::new_varkey(self.as_bytes()).expect("HMAC can take key of any size")
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

impl AsBytes for OutputHmac {
    fn as_bytes(&self) -> &[u8] {
        &self.code
    }
}
