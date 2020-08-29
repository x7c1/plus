mod error;
pub use error::Error;
pub use error::Result;

mod single;
pub use single::{env_var, EnvVar};

pub mod required {
    use std::str::FromStr;
    pub type Result<A> = crate::Result<A, <A as FromStr>::Err>;
}
pub mod optional {
    use std::str::FromStr;
    pub type Result<A> = crate::Result<Option<A>, <A as FromStr>::Err>;
}
