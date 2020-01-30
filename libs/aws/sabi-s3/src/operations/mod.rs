#[macro_use]
pub mod put_object;

use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Kind {
    PutObject,
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Kind::PutObject => write!(f, "PutObject"),
        }
    }
}
