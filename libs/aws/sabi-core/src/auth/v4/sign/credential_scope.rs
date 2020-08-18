use crate::auth::v4::chrono::DateStamp;
use crate::auth::v4::sign::ScopeTermination;
use crate::index::RegionCode;
use crate::index::ServiceCode;

#[derive(Debug)]
pub struct CredentialScope<'a> {
    pub date: DateStamp,
    pub region: &'a RegionCode,
    pub service: &'a ServiceCode,
    pub termination: ScopeTermination,
    raw: String,
}

impl<'a> CredentialScope<'a> {
    pub fn new(
        date: DateStamp,
        region: &'a RegionCode,
        service: &'a ServiceCode,
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

    pub fn v4(date: DateStamp, region: &'a RegionCode, service: &'a ServiceCode) -> Self {
        Self::new(date, region, service, ScopeTermination::Aws4Request)
    }

    pub fn as_str(&self) -> &str {
        &self.raw
    }
}
