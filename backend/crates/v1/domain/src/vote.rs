use crate::*;

#[derive(new, Getters)]
pub struct Vote {
    id: Id<Vote>,
    talk_id: Id<Talk>,
    to: Id<Player>,
    from: Id<Player>,
}
