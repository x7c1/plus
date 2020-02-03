/*
    see also:
    https://docs.aws.amazon.com/general/latest/gr/sigv4-create-string-to-sign.html
*/

mod algorithm;
pub use algorithm::Algorithm;

mod scope_termination;
pub use scope_termination::ScopeTermination;

mod string_to_sign;
pub use string_to_sign::StringToSign;

mod credential_scope;
pub use credential_scope::CredentialScope;
