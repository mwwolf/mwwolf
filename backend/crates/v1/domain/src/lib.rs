// #[macro_use]
extern crate derive_new;

#[macro_use]
extern crate derive_getters;

// #[macro_use]
extern crate async_trait;

mod error;
mod id;
mod result;

pub use error::*;
pub use id::Id;
// use libmww::*;
pub use result::{DomainResult, RepositoryResult};
