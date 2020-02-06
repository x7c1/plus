use crate::auth::account::AccessKey;
use crate::auth::v4::sign::CredentialScope;

#[derive(Debug)]
pub struct Credential(String);

impl Credential {
    pub fn new(access_key: &AccessKey, scope: &CredentialScope) -> Credential {
        Credential(format!(
            "{access_key}/{scope}",
            access_key = access_key.as_str(),
            scope = scope.as_str(),
        ))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
