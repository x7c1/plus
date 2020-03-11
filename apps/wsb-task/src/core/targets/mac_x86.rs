use crate::core::targets::BuildTarget;

pub struct MacX86;

impl BuildTarget for MacX86 {
    fn as_triple(&self) -> &str {
        "x86_64-apple-darwin"
    }
}
