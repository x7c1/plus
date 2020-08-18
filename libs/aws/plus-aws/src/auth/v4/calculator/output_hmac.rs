use crate::verbs::AsBytes;
use hex::ToHex;

pub struct OutputHmac {
    pub code: Vec<u8>,
}

impl OutputHmac {
    pub fn to_hex(&self) -> String {
        self.code.encode_hex()
    }
}

impl AsBytes for OutputHmac {
    fn as_bytes(&self) -> &[u8] {
        &self.code
    }
}
