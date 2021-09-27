#[derive(Getters)]
// TODO(ryutah): should be imple From trait
pub struct Room {
    id: String,
    player_count: u64,
    wolf_count: u64,
    host_player_id: String,
    all_players: Vec<String>,
    game_time: chrono::Duration,
    theme_kind: String,
}

#[derive(Getters)]
// TODO(ryutah): should be imple From trait
pub struct Game {
    id: String,
    room_id: String,
    theme_id: String,
    ended_at: chrono::DateTime<chrono_tz::Tz>,
    wolves: GroupDto,
    citizen: GroupDto,
    vote_box: VoteBoxDto,
    status: String,
}

#[derive(Getters)]
// TODO(ryutah): should be imple From trait
pub struct GroupDto {
    players: Vec<String>,
    word: String,
}

#[derive(Getters)]
// TODO(ryutah): should be imple From trait
pub struct VoteBoxDto {
    votes: Vec<VoteDto>,
}

#[derive(Getters)]
// TODO(ryutah): should be imple From trait
pub struct VoteDto {
    target: String,
    voter: String,
}
