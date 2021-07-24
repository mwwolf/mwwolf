use crate::*;
use chrono::*;

#[derive(new, Getters)]
pub struct Talk {
    id: Id<Talk>,
    theme_id: Id<Theme>,
    ended_at: DateTime<FixedOffset>,
    wolfs: Group,
    citizen: Group,
}

#[derive(new, Getters)]
pub struct Group {
    players: Vec<Player>,
    word: Word,
}
