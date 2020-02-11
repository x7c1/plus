use crate::auth::{AccessKey, SecretKey};
use crate::env::aws;
use crate::SabiResult;

#[derive(Debug)]
pub struct Credentials {
    pub access_key: AccessKey,
    pub secret_key: SecretKey,
}

impl Credentials {
    pub fn from_env() -> SabiResult<Credentials> {
        let credentials = Self::builder()
            // TODO:
            .access_key(aws::access_key().as_required()?)
            .secret_key(aws::secret_key().as_required()?)
            .build();

        Ok(credentials)
    }
    pub fn builder() -> CredentialsBuilder<(), ()> {
        CredentialsBuilder {
            access_key: (),
            secret_key: (),
        }
    }
}

pub struct CredentialsBuilder<AccessKeyType, SecretKeyType> {
    access_key: AccessKeyType,
    secret_key: SecretKeyType,
}

impl CredentialsBuilder<AccessKey, SecretKey> {
    pub fn build(self) -> Credentials {
        Credentials {
            access_key: self.access_key,
            secret_key: self.secret_key,
        }
    }
}

impl<AccessKeyType, SecretKeyType> CredentialsBuilder<AccessKeyType, SecretKeyType> {
    pub fn access_key(self, access_key: AccessKey) -> CredentialsBuilder<AccessKey, SecretKeyType> {
        CredentialsBuilder {
            access_key,
            secret_key: self.secret_key,
        }
    }
    pub fn secret_key(self, secret_key: SecretKey) -> CredentialsBuilder<AccessKeyType, SecretKey> {
        CredentialsBuilder {
            access_key: self.access_key,
            secret_key,
        }
    }
}
