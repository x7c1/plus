use std::io;
use std::io::Read;

pub trait BodyReceiver {
    fn receive_body_from<R: Read>(&mut self, body: R) -> io::Result<u64>;
}
