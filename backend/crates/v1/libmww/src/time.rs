use chrono::{DateTime, Utc};
use chrono_tz::Tz;

pub trait DateTimeGen {
    fn now(&self) -> DateTime<Tz>;
}

pub struct DateTimeGenImpl;

impl DateTimeGen for DateTimeGenImpl {
    fn now(&self) -> DateTime<Tz> {
        Utc::now().with_timezone(&chrono_tz::Japan)
    }
}
