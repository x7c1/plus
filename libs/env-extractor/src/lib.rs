mod error;
pub use error::Error;
pub use error::Result;

mod single;
pub use single::EnvVar;

use std::str::FromStr;
pub type RequiredResult<A> = Result<A, <A as FromStr>::Err>;
pub type OptionalResult<A> = Result<Option<A>, <A as FromStr>::Err>;
