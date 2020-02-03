use crate::auth::v4::chrono::DateStamp;
use crate::auth::v4::sign::ScopeTermination;
use crate::index::RegionCode;
use crate::index::ServiceCode;

#[derive(Debug)]
pub struct CredentialScope {
    pub date: DateStamp,
    pub region: RegionCode,
    pub service: ServiceCode,
    pub termination: ScopeTermination,
    raw: String,
}

impl CredentialScope {
    pub fn from(
        date: DateStamp,
        region: RegionCode,
        service: ServiceCode,
        termination: ScopeTermination,
    ) -> Self {
        let raw = format!(
            "{date}/{region}/{service}/{termination}",
            date = date.as_str(),
            region = region.as_str(),
            service = service.as_str(),
            termination = termination.as_str(),
        );
        CredentialScope {
            date,
            region,
            service,
            termination,
            raw,
        }
    }
    pub fn as_str(&self) -> &str {
        &self.raw
    }
}