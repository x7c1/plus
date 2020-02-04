use crate::auth::v4::key::CanGenerateHmac;
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
