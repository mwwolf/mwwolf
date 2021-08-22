#[cfg(test)]
use mockall::{automock, predicate::*};

mod error;
mod id;
mod player;
mod result;
mod room;
mod selection;
mod talk;
mod theme;
mod vote;

use crate::libmww::*;
pub use error::*;
pub use id::Id;

pub use player::*;
pub use player::*;
pub use room::*;
pub use selection::*;
pub use talk::*;
pub use theme::*;
pub use vote::*;

pub use result::{DomainResult, RepositoryResult};
