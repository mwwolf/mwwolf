pub mod di;
pub mod domain;
pub mod infrastructure;
pub mod presentation;
pub mod usecase;

mod libmww;
#[cfg(test)]
mod testmww;

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

#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate async_graphql;

#[macro_use]
extern crate nameof;

use async_std::sync::*;
#[cfg(test)]
use mockall::{automock, mock, predicate::*};
