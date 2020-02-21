use std::io;
use std::io::{Read, Write};

pub trait Copier {
    fn copy_to<W: Write>(self, out: &mut W) -> io::Result<u64>;
}

impl<R> Copier for R
where
    R: Read,
{
    fn copy_to<W: Write>(mut self, out: &mut W) -> io::Result<u64> {
        io::copy(&mut self, out)
    }
}
