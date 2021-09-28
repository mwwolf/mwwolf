use chrono::*;

#[derive(PartialEq, Debug, new, Getters)]
pub struct RoomCreate {
    player_count: usize,
    wolf_count: usize,
    host_player_id: String,
    game_minutes: Duration,
    theme_kind: String,
}

#[derive(PartialEq, Debug, new, Getters)]
pub struct StartGame {
    room_id: String,
}

#[derive(PartialEq, Debug, new, Getters)]
pub struct RoomJoin {
    room_id: String,
    player_id: String,
}
