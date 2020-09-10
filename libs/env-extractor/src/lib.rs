mod error;

pub use error::Error;
pub use error::Result;

mod var;

pub use var::{env_var, EnvVar};

pub mod required {
    use std::str::FromStr;

    pub type Result<A> = crate::Result<A, <A as FromStr>::Err>;
}

pub mod optional {
    use std::str::FromStr;

    pub type Result<A> = crate::Result<Option<A>, <A as FromStr>::Err>;
}

#[cfg(test)]
mod tests {
    mod test_optional {
        use crate::env_var;
        use std::str::FromStr;

        #[test]
        fn return_none_if_not_found() {
            let sample = Sample {
                x: env_var("unknown_variable_name").as_optional().unwrap(),
            };
            assert_eq!(sample.x.is_none(), true);
        }

        #[test]
        fn can_call_from_str_if_defined() {
            let sample: Option<Sample> = env_var("PATH").as_optional().unwrap();
            assert_eq!(sample.is_some(), true);
        }

        #[derive(Debug)]
        struct Sample {
            x: Option<String>,
        }

        impl FromStr for Sample {
            type Err = <String as FromStr>::Err;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Sample {
                    x: Some(s.to_string()),
                })
            }
        }
    }

    mod test_required {
        use crate::Error::NotPresent;
        use crate::{env_var, required};
        use std::str::FromStr;

        #[test]
        fn return_error_if_not_found() {
            let x: required::Result<String> = env_var("unknown_variable_name").as_required();
            match x {
                Err(NotPresent(key)) => assert_eq!(key, "unknown_variable_name"),
                Ok(_) => assert!(false, "unexpected success"),
                _ => assert!(false, "unexpected error type"),
            }
        }

        #[test]
        fn return_value_if_key_found() {
            let sample = Sample {
                x: env_var("PATH").as_required().unwrap(),
            };
            assert_eq!(sample.x.is_empty(), false);
        }

        #[test]
        fn can_call_from_str_if_defined() {
            let sample: Sample = env_var("PATH").as_required().unwrap();
            assert_eq!(sample.x.is_empty(), false);
        }

        #[derive(Debug)]
        struct Sample {
            x: String,
        }

        impl FromStr for Sample {
            type Err = <String as FromStr>::Err;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Sample { x: s.to_string() })
            }
        }
    }
}
