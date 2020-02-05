use crate::auth::SecretKey;
use crate::verbs::AsBytes;
use hmac::{Hmac, Mac};
use sha2::Sha256;

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
