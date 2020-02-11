use crate::AsStr;
use crate::Characters;

pub trait MultiByte: AsStr {
    fn length(&self) -> usize {
        self.as_str().chars().count()
    }

    fn tail(&self, length: usize) -> &str {
        let original = self.as_str();
        let head_length = {
            let char_counts = self.length() - length;
            original
                .chars()
                .skip(char_counts)
                .fold(0, |size, x| size + x.len_utf8())
        };
        let start = original.len() - head_length;
        &original[start..]
    }
}

impl Characters for dyn MultiByte {
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
