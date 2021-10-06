#[derive(PartialEq, Debug, new, Getters)]
pub struct RoomCreate {
    player_count: usize,
    wolf_count: usize,
    host_player_id: String,
    game_minutes: u32,
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

#[derive(PartialEq, Debug, new, Getters)]
pub struct RoomLeave {
    room_id: String,
    player_id: String,
}

#[derive(PartialEq, Debug, new, Getters)]
pub struct Vote {
    target: String,
    voter: String,
}
