use crate::SabiResult;
use hex::ToHex;
use sha2::{Digest, Sha256};
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Debug)]
pub struct PayloadHash(String);

impl PayloadHash {
    pub fn new<A: Into<String>>(value: A) -> Self {
        PayloadHash(value.into())
    }

    pub fn empty() -> Self {
        Self::new(
            /*
                rf.
                https://docs.aws.amazon.com/general/latest/gr/sigv4-create-canonical-request.html
            */
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        )
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<&File> for PayloadHash {
    type Error = crate::Error;

    fn try_from(file: &File) -> SabiResult<Self> {
        let mut sha = Sha256::default();
        let mut reader = BufReader::new(file);
        let mut buffer = [0; 4];
        loop {
            /*
                rf.
                https://qiita.com/fujitayy/items/12a80560a356607da637
            */
            match reader.read(&mut buffer)? {
                0 => break,
                n => sha.input(&buffer[..n]),
            }
        }
        let hex: String = sha.result().as_slice().encode_hex();
        let hash = Self::new(hex);
        Ok(hash)
    }
}
