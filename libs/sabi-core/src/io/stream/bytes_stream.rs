use bytes::{Bytes, BytesMut};
use futures_util::{stream::Stream, TryStreamExt};
use std::io;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

pub fn from_file(file: File) -> impl Stream<Item = io::Result<Bytes>> {
    let decoder = BytesCodec::new();
    let stream = FramedRead::new(file, decoder).map_ok(BytesMut::freeze);
    stream
}
