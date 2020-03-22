use crate::core::targets::BuildTarget;

#[derive(Clone, Debug)]
pub struct MacX86;

impl BuildTarget for MacX86 {
    fn as_triple(&self) -> &str {
        "x86_64-apple-darwin"
    }
    fn as_abbr(&self) -> &str {
        "macos_x86"
    }
}
