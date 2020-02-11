pub trait AsStr {
    fn as_str(&self) -> &str;
}

pub trait Characters {
    fn as_str(&self) -> &str;
    fn length(&self) -> usize;
    fn tail(&self, length: usize) -> &str;
}
