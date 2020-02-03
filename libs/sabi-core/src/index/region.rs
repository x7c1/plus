
#[derive(Debug)]
pub enum AwsRegion {
    ApNorthEast1,
    UsEast1,
    Unknown(String),
}

impl AwsRegion {
    pub fn as_str(&self) -> &str {
        match self {
            AwsRegion::ApNorthEast1 => "ap-northeast-1",
            AwsRegion::UsEast1 => "us-east-1",
            AwsRegion::Unknown(label) => &label,
        }
    }
}

impl<'a> Into<&'a [u8]> for &'a AwsRegion {
    fn into(self) -> &'a [u8] {
        self.as_str().as_bytes()
    }
}
