pub mod command;
pub mod dto;

mod error;

pub use error::*;

use crate::domain;
use crate::libmww;

pub type Result<T> = domain::DomainResult<T>;

trait Room {
    fn create(&self, command: command::RoomCreate) -> Result<dto::Room>;
    fn delete(&self, room_id: impl Into<String>) -> Result<()>;
    fn join(&self, command: command::RoomJoin) -> Result<dto::Room>;
    fn leave(&self, palyer_id: impl Into<String>) -> Result<dto::Room>;
    fn start_game(&self, command: command::StartGame) -> Result<dto::Game>;
}

pub struct Vote;

trait Game {
    fn start_talk(&self) -> Result<dto::Game>;
    fn end_talk(&self) -> Result<dto::Game>;
    fn start_vote(&self) -> Result<dto::Game>;
    fn end_vote(&self) -> Result<dto::Game>;
    fn vote(&self, command: Vote) -> Result<dto::Game>;
}
