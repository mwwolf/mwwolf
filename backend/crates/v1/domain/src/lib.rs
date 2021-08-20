#[macro_use]
extern crate derive_new;

#[macro_use]
extern crate derive_getters;

#[macro_use]
extern crate strum_macros;

#[macro_use]
extern crate libmww_macro;

#[cfg(test)]
#[macro_use]
extern crate test_case;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[macro_use]
extern crate async_trait;

mod error;
mod id;
mod player;
mod result;
mod room;
mod selection;
mod talk;
mod theme;
mod vote;

pub use error::*;
pub use id::Id;
use libmww::*;
//
pub use player::*;
pub use player::*;
pub use room::*;
pub use selection::*;
pub use talk::*;
pub use theme::*;
pub use vote::*;

pub use result::{DomainResult, RepositoryResult};
