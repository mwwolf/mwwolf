use super::*;
trait Game {
    fn start_talk(&self) -> Result<dto::Game>;
    fn end_talk(&self) -> Result<dto::Game>;
    fn start_vote(&self) -> Result<dto::Game>;
    fn end_vote(&self) -> Result<dto::Game>;
    fn vote(&self, command: command::Vote) -> Result<dto::Game>;
}
