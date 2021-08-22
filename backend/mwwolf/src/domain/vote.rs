use super::*;

#[derive(new, Getters)]
pub struct Vote {
    id: Id<Vote>,
    talk_id: Id<Talk>,
    target: Id<Player>,
    voter: Id<Player>,
}
