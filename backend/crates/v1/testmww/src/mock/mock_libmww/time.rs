use ::libmww::time;

use crate::mock::*;
use chrono::DateTime;
use chrono_tz::Tz;

mock! {
    pub DateTimeGen{}

    impl time::DateTimeGen for DateTimeGen{
        fn now(&self)->DateTime<Tz>;
    }
}
