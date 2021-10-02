pub mod command;
pub mod dto;

mod error;
mod game;
mod room;

pub use error::*;
pub use game::*;
pub use room::*;

use crate::domain;
use crate::libmww;

pub type Result<T> = domain::DomainResult<T>;
