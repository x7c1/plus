use chrono::{DateTime, Utc};

/// > The date must be in the YYYYMMDD format.
/// > Note that the date does not include a time value.
///
/// ## Example
///  * 20150830
///
/// ## See also
///  * [Task 2: Create a String to Sign for Signature Version 4 - AWS General Reference](https://docs.aws.amazon.com/general/latest/gr/sigv4-create-string-to-sign.html)
///
#[derive(Debug)]
pub struct DateStamp(String);

impl DateStamp {
    pub fn new<A: Into<String>>(key: A) -> DateStamp {
        DateStamp(key.into())
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl<'a> Into<&'a [u8]> for &'a DateStamp {
    fn into(self) -> &'a [u8] {
        self.0.as_bytes()
    }
}

impl From<&DateTime<Utc>> for DateStamp {
    fn from(time: &DateTime<Utc>) -> Self {
        let pattern = "%Y%m%d";
        DateStamp::new(time.format(pattern).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::DateStamp;
    use chrono::{TimeZone, Utc};

    #[test]
    fn it_works() {
        let time = Utc.ymd(2020, 2, 3).and_hms(2, 3, 4);
        let stamp = DateStamp::from(&time);
        assert_eq!(stamp.as_str(), "20200203");
    }
}
