use crate::AsStr;
use crate::Characters;

pub trait MultiByte: AsStr {
    fn length(&self) -> usize {
        self.as_str().chars().count()
    }

    fn tail(&self, length: usize) -> &str {
        let original = self.as_str();
        let pair = {
            let start = self.length() - length;
            original.char_indices().nth(start)
        };
        if let Some((index, _)) = pair {
            &original[index..]
        } else {
            ""
        }
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
