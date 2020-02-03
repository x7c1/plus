/// see also:
///  * [AWS Service Endpoints - AWS General Reference](https://docs.aws.amazon.com/general/latest/gr/rande.html)
///
#[derive(Debug)]
pub enum RegionCode {
    ApNorthEast1,
    UsEast1,
    Unknown(String),
}

impl RegionCode {
    pub fn as_str(&self) -> &str {
        match self {
            RegionCode::ApNorthEast1 => "ap-northeast-1",
            RegionCode::UsEast1 => "us-east-1",
            RegionCode::Unknown(code) => &code,
        }
    }
}

impl<'a> Into<&'a [u8]> for &'a RegionCode {
    fn into(self) -> &'a [u8] {
        self.as_str().as_bytes()
    }
}
