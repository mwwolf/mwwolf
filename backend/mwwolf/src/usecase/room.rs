use super::*;

trait Room {
    fn create(&self, command: command::RoomCreate) -> Result<dto::Room>;
    fn delete(&self, room_id: &str) -> Result<()>;
    fn join(&self, command: command::RoomJoin) -> Result<dto::Room>;
    fn leave(&self, palyer_id: &str) -> Result<dto::Room>;
    fn start_game(&self, command: command::StartGame) -> Result<dto::Game>;
}

#[derive(new, Getters)]
pub struct RoomImpl {}
