use crate::verbs::AsBytes;
use crate::SabiResult;
use std::str::FromStr;

/// see also:
///  * [AWS Service Endpoints - AWS General Reference](https://docs.aws.amazon.com/general/latest/gr/rande.html)
///
#[derive(Clone, Debug)]
pub enum RegionCode {
    ApNorthEast1,
    UsEast1,
    Unknown(String),
}

impl RegionCode {
    pub fn from_env() -> SabiResult<Option<RegionCode>> {
        // todo:
        unimplemented!()
    }

    pub fn new<A: Into<String>>(key: A) -> Self {
        Self::Unknown(key.into())
    }

    pub fn as_str(&self) -> &str {
        match self {
            RegionCode::ApNorthEast1 => "ap-northeast-1",
            RegionCode::UsEast1 => "us-east-1",
            RegionCode::Unknown(code) => &code,
        }
    }
}

impl AsBytes for RegionCode {
    fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}

impl AsBytes for &RegionCode {
    fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}

impl FromStr for RegionCode {
    type Err = crate::Error;

    fn from_str(s: &str) -> SabiResult<Self> {
        Ok(Self::new(s))
    }
}
