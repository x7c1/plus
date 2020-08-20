pub trait AsBytes {
    fn as_bytes(&self) -> &[u8];
}

impl AsBytes for &[u8] {
    fn as_bytes(&self) -> &[u8] {
        self
    }
}

impl AsBytes for Vec<u8> {
    fn as_bytes(&self) -> &[u8] {
        self
    }
}
