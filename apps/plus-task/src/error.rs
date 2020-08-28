extern crate failure;

use crate::tasks::release_libraries::crates_io::CargoTomlPackage;
use std::fmt::Debug;
use std::string;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    PackageAlreadyPublished(CargoTomlPackage),
    ClapTaskError(clap_task::Error),
    ClapExtractorError(Box<dyn Debug>),
    CrateVersionNotFound(CargoTomlPackage),
    SerdeJsonError(serde_json::Error),
    ShellworkError(shellwork::Error),
    StdIoError(std::io::Error),
    StringFromUtf8Error(string::FromUtf8Error),
    InvalidCargoToml(toml::de::Error),
    UnknownBuildTarget(String),
}

impl From<clap_task::Error> for Error {
    fn from(e: clap_task::Error) -> Self {
        Error::ClapTaskError(e)
    }
}

impl<A: Debug> From<clap_extractor::Error<A>> for Error
where
    A: 'static,
{
    fn from(e: clap_extractor::Error<A>) -> Self {
        Error::ClapExtractorError(Box::new(e))
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::SerdeJsonError(e)
    }
}

impl From<shellwork::Error> for Error {
    fn from(e: shellwork::Error) -> Self {
        Error::ShellworkError(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::StdIoError(e)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(e: string::FromUtf8Error) -> Self {
        Error::StringFromUtf8Error(e)
    }
}
