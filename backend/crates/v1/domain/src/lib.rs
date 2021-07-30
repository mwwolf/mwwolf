#[macro_use]
extern crate derive_new;

#[macro_use]
extern crate derive_getters;

#[macro_use]
extern crate strum_macros;

#[macro_use]
extern crate libmww_macro;

// #[macro_use]
extern crate async_trait;

mod error;
mod id;
mod player;
mod result;
mod selection;
mod talk;
mod theme;
mod vote;

pub use error::*;
pub use id::Id;
// use libmww::*;
//
pub use player::*;
pub use player::*;
pub use selection::*;
pub use talk::*;
pub use theme::*;
pub use vote::*;

pub use result::{DomainResult, RepositoryResult};
