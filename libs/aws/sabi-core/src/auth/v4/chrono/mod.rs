mod amz_date;
pub use amz_date::AmzDate;

mod date_stamp;
use chrono::{DateTime, Utc};
pub use date_stamp::DateStamp;

pub fn now() -> DateTime<Utc> {
    Utc::now()
}
