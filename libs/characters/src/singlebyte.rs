use crate::AsStr;
use crate::Characters;

pub trait SingleByte: AsStr {
    fn length(&self) -> usize {
        self.as_str().len()
    }

    fn tail(&self, length: usize) -> &str {
        let start = self.length() - length;
        &self.as_str()[start..]
    }
}

impl Characters for dyn SingleByte {
    fn as_str(&self) -> &str {
        Self::as_str(self)
    }

    fn length(&self) -> usize {
        Self::length(self)
    }

    fn tail(&self, length: usize) -> &str {
        Self::tail(self, length)
    }
}
