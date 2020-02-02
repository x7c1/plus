use chrono::{DateTime, Utc};

/// > The time stamp must be in UTC and in the following ISO 8601 format: YYYYMMDD'T'HHMMSS'Z'.
/// > Do not include milliseconds in the time stamp.
///
/// ## Example
///  * 20150830T123600Z
///
/// ## See also
///  * [Handling Dates in Signature Version 4 - AWS General Reference](https://docs.aws.amazon.com/general/latest/gr/sigv4-date-handling.html)
///
#[derive(Debug)]
pub struct AmzDate(String);

impl AmzDate {
    pub fn new<A: Into<String>>(value: A) -> AmzDate {
        AmzDate(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<&DateTime<Utc>> for AmzDate {
    fn from(time: &DateTime<Utc>) -> Self {
        let pattern = "%Y%m%dT%H%M%SZ";
        AmzDate::new(time.format(pattern).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn it_works() {
        let time = Utc.ymd(2020, 2, 3).and_hms(4, 50, 6);
        let date = AmzDate::from(&time);
        assert_eq!(date.as_str(), "20200203T045006Z");
    }
}
