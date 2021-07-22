#[macro_use]
extern crate derive_new;

#[macro_use]
extern crate derive_getters;

#[macro_use]
extern crate strum_macros;

// #[macro_use]
extern crate async_trait;

mod error;
mod id;
mod player;
mod result;

pub use error::*;
pub use id::Id;
// use libmww::*;
pub use player::*;
pub use result::{DomainResult, RepositoryResult};
