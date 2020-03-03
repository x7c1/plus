use crate::io::stream::bytes_stream;
use crate::SabiResult;
use bytes::Bytes;
use futures_util::{future, stream::Stream, TryStreamExt};
use hex::ToHex;
use sha2::{Digest, Sha256};
use std::io;
use tokio::fs;

#[derive(Debug)]
pub struct HashedPayload(String);

impl HashedPayload {
    pub fn new<A: Into<String>>(value: A) -> Self {
        HashedPayload(value.into())
    }

    pub fn empty() -> Self {
        Self::new("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub async fn from_file(file: fs::File) -> SabiResult<Self> {
        let stream = bytes_stream::from_file(file);
        let hash = calculate(stream).await?;
        Ok(hash)
    }
}

async fn calculate<S>(stream: S) -> SabiResult<HashedPayload>
where
    S: Stream<Item = io::Result<Bytes>>,
    S: Unpin,
{
    let mut sha = Sha256::default();
    stream
        .try_for_each(|item: Bytes| {
            sha.input(item);
            future::ok(())
        })
        .await?;

    let hex: String = sha.result().as_slice().encode_hex();
    let hash = HashedPayload::new(hex);
    Ok(hash)
}
