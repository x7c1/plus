use crate::SabiResult;
use bytes::{Bytes, BytesMut};
use futures_util::future;
use futures_util::stream::Stream;
use futures_util::TryStreamExt;
use hex::ToHex;
use sha2::{Digest, Sha256};
use std::convert::TryFrom;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use std::task::{Context, Poll};
use tokio::fs;
use tokio::future::poll_fn;
use tokio::io::AsyncRead;
use tokio_util::codec::Decoder;
use tokio_util::codec::{BytesCodec, FramedRead};

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
        let stream = FramedRead::new(file, BytesCodec::new()).map_ok(BytesMut::freeze);
        let hash = calculate(stream).await;
        Ok(hash)
    }
}

impl TryFrom<&File> for HashedPayload {
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

async fn calculate<S>(mut stream: S) -> HashedPayload
where
    S: Stream<Item = Result<Bytes, io::Error>>,
    S: Unpin,
{
    let mut sha = Sha256::default();

    let xs2 = stream.try_for_each(|item: Bytes| {
        eprintln!("============== : {}", item.len());
        sha.input(item);
        future::ready(Ok({}))
    });
    xs2.await;

    let hex: String = sha.result().as_slice().encode_hex();
    HashedPayload::new(hex)
}
