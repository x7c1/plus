use std::io;
use std::io::{BufReader, Read, Write};

// copied from private std::io::DEFAULT_BUF_SIZE
const DEFAULT_BUF_SIZE: usize = 8 * 1024;

pub trait RichReader {
    fn write_gradually_to<W: Write>(self, out: &mut W) -> io::Result<()>;
}

impl<R> RichReader for R
where
    R: Read,
{
    fn write_gradually_to<W: Write>(self, out: &mut W) -> io::Result<()> {
        let mut reader = BufReader::new(self);
        let mut buffer = [0; DEFAULT_BUF_SIZE];
        loop {
            match reader.read(&mut buffer)? {
                0 => break,
                n => {
                    let buffer: &[u8] = &buffer[..n];
                    out.write_all(buffer)?;
                }
            }
        }
        out.flush()
    }
}
