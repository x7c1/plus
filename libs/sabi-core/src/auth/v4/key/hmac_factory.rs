use crate::auth::v4::key::{CanGenerateHmac, OutputHmac};
use crate::verbs::AsBytes;
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
