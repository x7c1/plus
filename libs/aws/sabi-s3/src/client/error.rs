pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "CredentialsError > {}", 0)]
    CredentialsError(sabi_core::Error),

    #[fail(display = "RegionCodeError > {}", 0)]
    RegionCodeError(sabi_core::Error),
}
