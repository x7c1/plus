use crate::{actions, client, core};

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "actions::Error > {}", 0)]
    ActionsError(actions::Error),

    #[fail(display = "client::Error > {}", 0)]
    ClientError(client::Error),

    #[fail(display = "core::Error > {}", 0)]
    CoreError(core::Error),
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
