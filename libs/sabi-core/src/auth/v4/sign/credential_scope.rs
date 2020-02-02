use crate::auth::v4::chrono::DateStamp;
use crate::auth::v4::sign::{ScopeRegion, ScopeService, ScopeTermination};

#[derive(Debug)]
pub struct CredentialScope {
    pub date: DateStamp,
    pub region: ScopeRegion,
    pub service: ScopeService,
    pub termination: ScopeTermination,
}
