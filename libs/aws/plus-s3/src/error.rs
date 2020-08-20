use crate::{actions, client, core, internal};

/// see also: libs/aws/plus-s3-macros
/// plus-s3-macros helps to integrate plus_s3::*::Error into one plus-s3::Error.

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "actions::Error > {}", 0)]
    ActionsError(actions::Error),

    #[fail(display = "client::Error > {}", 0)]
    ClientError(client::Error),

    #[fail(display = "core::Error > {}", 0)]
    CoreError(core::Error),

    #[fail(display = "internal::Error > {}", 0)]
    InternalError(internal::Error),
}

impl From<actions::Error> for Error {
    fn from(e: actions::Error) -> Self {
        Error::ActionsError(e)
    }
}

impl From<client::Error> for Error {
    fn from(e: client::Error) -> Self {
        Error::ClientError(e)
    }
}

impl From<core::Error> for Error {
    fn from(e: core::Error) -> Self {
        Error::CoreError(e)
    }
}

impl From<internal::Error> for Error {
    fn from(e: internal::Error) -> Self {
        Error::InternalError(e)
    }
}
