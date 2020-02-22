use std::io::Read;

pub trait BodyReceiver {
    type Err;
    fn receive_body_from<R: Read>(&mut self, body: R) -> Result<u64, Self::Err>;
}
