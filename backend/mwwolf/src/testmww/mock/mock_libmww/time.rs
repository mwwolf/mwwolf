use crate::libmww::time;

use crate::*;
use chrono::DateTime;
use chrono_tz::Tz;

mock! {
    pub DateTimeGen{}

    impl time::DateTimeGen for DateTimeGen{
        fn now(&self)->DateTime<Tz>;
    }
}
