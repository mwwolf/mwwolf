#[macro_use]
extern crate async_trait;

#[cfg(feature = "test")]
use mockall::{automock, predicate::*};

pub mod database;
pub mod id;
pub mod random;
pub mod time;
