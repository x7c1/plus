use std::io;
use std::io::{BufReader, Read, Write};

pub trait RichReader {
    fn write_gradually_to<W: Write>(self, out: &mut W) -> io::Result<()>;
}

impl<R> RichReader for R
where
    R: Read,
{
    fn write_gradually_to<W: Write>(self, out: &mut W) -> io::Result<()> {
        let mut reader = BufReader::new(self);

        // todo: use DEFAULT_BUF_SIZE?
        let mut buffer = [0; 8 * 1024];
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
