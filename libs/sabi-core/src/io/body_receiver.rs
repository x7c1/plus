use bytes::Bytes;
use futures_util::stream::Stream;

#[async_trait]
pub trait BodyReceiver {
    type Err;

    async fn receive_body_from<S>(&mut self, body: S) -> Result<usize, Self::Err>
    where
        S: Stream<Item = Result<Bytes, Self::Err>>,
        S: Send;
}
