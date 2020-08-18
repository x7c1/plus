pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "CredentialsError > {}", 0)]
    CredentialsError(plus_aws::Error),

    #[fail(display = "RegionCodeError > {}", 0)]
    RegionCodeError(plus_aws::Error),
}
