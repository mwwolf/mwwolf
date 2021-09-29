use crate::domain;
#[derive(Getters, PartialEq, Debug)]
pub struct Room {
    id: String,
    player_count: usize,
    wolf_count: usize,
    host_player_id: String,
    all_players: Vec<String>,
    game_time: chrono::Duration,
    theme_kind: String,
}

impl From<domain::Room> for Room {
    fn from(room: domain::Room) -> Self {
        Self {
            id: room.id().raw_id().clone(),
            player_count: room.player_count().raw_player_count().to_owned(),
            wolf_count: room.wolf_count().raw_count().to_owned(),
            host_player_id: room.host_player_id().raw_id().to_owned(),
            all_players: room
                .all_players()
                .iter()
                .map(|id| id.raw_id().to_owned())
                .collect(),
            game_time: room.game_time().raw_minutes().to_owned(),
            theme_kind: room.theme_kind().raw_kind().to_owned(),
        }
    }
}

#[derive(Getters, PartialEq, Debug)]
pub struct Game {
    id: String,
    room_id: String,
    theme_id: String,
    ended_at: chrono::DateTime<chrono_tz::Tz>,
    wolves: Group,
    citizen: Group,
    vote_box: VoteBox,
    status: String,
}

impl From<domain::Game> for Game {
    fn from(game: domain::Game) -> Self {
        Self {
            id: game.id().to_owned().into(),
            room_id: game.room_id().to_owned().into(),
            theme_id: game.theme_id().to_owned().into(),
            ended_at: game.ended_at().to_owned(),
            wolves: game.wolves().to_owned().into(),
            citizen: game.citizen().to_owned().into(),
            vote_box: game.vote_box().to_owned().into(),
            status: game.status().to_string(),
        }
    }
}

#[derive(Getters, PartialEq, Debug)]
pub struct Group {
    players: Vec<String>,
    word: String,
}

impl From<domain::WolfGroup> for Group {
    fn from(wolf_group: domain::WolfGroup) -> Self {
        Self {
            players: wolf_group
                .players()
                .iter()
                .map(|id| id.to_owned().into())
                .collect(),
            word: wolf_group.word().raw().to_owned(),
        }
    }
}

impl From<domain::CitizenGroup> for Group {
    fn from(citizen_group: domain::CitizenGroup) -> Self {
        Self {
            players: citizen_group
                .players()
                .iter()
                .map(|id| id.to_owned().into())
                .collect(),
            word: citizen_group.word().raw().to_owned(),
        }
    }
}

#[derive(Getters, PartialEq, Debug)]
pub struct VoteBox {
    votes: Vec<Vote>,
}

impl From<domain::VoteBox> for VoteBox {
    fn from(vote_box: domain::VoteBox) -> Self {
        Self {
            votes: vote_box
                .votes()
                .iter()
                .map(|vote| vote.to_owned().into())
                .collect(),
        }
    }
}

#[derive(Getters, PartialEq, Debug)]
pub struct Vote {
    target: String,
    voter: String,
}

impl From<domain::Vote> for Vote {
    fn from(vote: domain::Vote) -> Self {
        Self {
            target: vote.target().to_owned().into(),
            voter: vote.voter().to_owned().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::DateTime;
    use chrono::TimeZone;
    use chrono_tz::Tz;
    use test_case::test_case;

    #[test_case(
        domain::Vote::new(
            domain::Id::new("target1"),
            domain::Id::new("voter1"),
            )
        => Vote{target:"target1".into(),voter:"voter1".into()}
        )]
    #[test_case(
        domain::Vote::new(
            domain::Id::new("target2"),
            domain::Id::new("voter2"),
            )
        => Vote{target:"target2".into(),voter:"voter2".into()}
        )]
    fn vote_from_domain_works(d: domain::Vote) -> Vote {
        Vote::from(d)
    }

    #[test_case(
        domain::VoteBox::new(vec![
            domain::Vote::new(
                domain::Id::new("target1"),
                domain::Id::new("voter1"),
                ),
            domain::Vote::new(
                domain::Id::new("target2"),
                domain::Id::new("voter2"),
                ),
        ])
        => VoteBox{
            votes:vec![
                Vote{target:"target1".into(),voter:"voter1".into()},
                Vote{target:"target2".into(),voter:"voter2".into()},
            ],
        })]
    #[test_case(
        domain::VoteBox::new(vec![
            domain::Vote::new(
                domain::Id::new("target3"),
                domain::Id::new("voter3"),
                ),
            domain::Vote::new(
                domain::Id::new("target4"),
                domain::Id::new("voter4"),
                ),
        ])
        => VoteBox{
            votes:vec![
                Vote{target:"target3".into(),voter:"voter3".into()},
                Vote{target:"target4".into(),voter:"voter4".into()},
            ],
        })]
    fn vote_box_from_domain_works(d: domain::VoteBox) -> VoteBox {
        VoteBox::from(d)
    }

    #[test_case(domain::WolfGroup::new(
            vec![domain::Id::new("player1")],
            domain::Word::try_new("hoge").unwrap())
        => Group{
                players:vec!["player1".to_owned()],
                word:"hoge".to_owned(),
                })]
    #[test_case(domain::WolfGroup::new(
            vec![domain::Id::new("player2")],
            domain::Word::try_new("fuga").unwrap())
        => Group{
                players:vec!["player2".to_owned()],
                word:"fuga".to_owned(),
                })]
    fn group_from_domain_wolf_group_works(d: domain::WolfGroup) -> Group {
        Group::from(d)
    }

    #[test_case(domain::CitizenGroup::new(
            vec![domain::Id::new("player1")],
            domain::Word::try_new("hoge").unwrap())
        => Group{
                players:vec!["player1".to_owned()],
                word:"hoge".to_owned(),
                })]
    #[test_case(domain::CitizenGroup::new(
            vec![domain::Id::new("player2")],
            domain::Word::try_new("fuga").unwrap())
        => Group{
                players:vec!["player2".to_owned()],
                word:"fuga".to_owned(),
                })]
    fn group_from_domain_citizen_group_works(d: domain::CitizenGroup) -> Group {
        Group::from(d)
    }

    #[test_case(
        domain::Game::new(
            domain::Id::new("game1"),
            domain::Id::new("room1"),
            domain::Id::new("theme1"),
            datetime(2020,3,4,3,23,2),
            domain::WolfGroup::new(
                vec![domain::Id::new("player1")],
                domain::Word::try_new("hoge").unwrap()),
            domain::CitizenGroup::new(
                vec![domain::Id::new("player2")],
                domain::Word::try_new("fuga").unwrap()),
            domain::VoteBox::new(vec![]),
            domain::GameStatus::Voting)
        => Game {
            id: "game1".into(),
            room_id: "room1".into(),
            theme_id: "theme1".into(),
            ended_at: datetime(2020, 3, 4, 3, 23, 2),
            wolves: Group {
                players: vec!["player1".to_owned()],
                word: "hoge".to_owned(),
            },
            citizen: Group {
                players: vec!["player2".to_owned()],
                word: "fuga".to_owned(),
            },
            vote_box: VoteBox { votes: vec![] },
            status: domain::GameStatus::Voting.to_string(),
        })]
    #[test_case(
        domain::Game::new(
            domain::Id::new("game2"),
            domain::Id::new("room2"),
            domain::Id::new("theme2"),
            datetime(2020,3,4,3,23,2),
            domain::WolfGroup::new(
                vec![domain::Id::new("player2")],
                domain::Word::try_new("hoge2").unwrap()),
            domain::CitizenGroup::new(
                vec![domain::Id::new("player3")],
                domain::Word::try_new("fuga2").unwrap()),
            domain::VoteBox::new(vec![]),
            domain::GameStatus::Voting)
        => Game {
            id: "game2".into(),
            room_id: "room2".into(),
            theme_id: "theme2".into(),
            ended_at: datetime(2020, 3, 4, 3, 23, 2),
            wolves: Group {
                players: vec!["player2".to_owned()],
                word: "hoge2".to_owned(),
            },
            citizen: Group {
                players: vec!["player3".to_owned()],
                word: "fuga2".to_owned(),
            },
            vote_box: VoteBox { votes: vec![] },
            status: domain::GameStatus::Voting.to_string(),
        })]
    fn game_from_domain_works(d: domain::Game) -> Game {
        Game::from(d)
    }

    #[test_case(
        domain::WolfGroup::new(
            vec![domain::Id::new("player1")],
            domain::Word::try_new("hoge1").unwrap(),
        )
        =>
        Group {
            players: vec!["player1".to_owned()],
            word: "hoge1".to_owned(),
        }
    )]
    #[test_case(
        domain::WolfGroup::new(
            vec![domain::Id::new("player2"), domain::Id::new("player3")],
            domain::Word::try_new("hoge2").unwrap(),
        )
        =>
        Group {
            players: vec!["player2".to_owned(), "player3".to_owned()],
            word: "hoge2".to_owned(),
        }
    )]
    fn group_from_wolf_group_domain_works(d: domain::WolfGroup) -> Group {
        Group::from(d)
    }

    #[test_case(
        domain::CitizenGroup::new(
            vec![domain::Id::new("player1")],
            domain::Word::try_new("hoge1").unwrap(),
        )
        =>
        Group {
            players: vec!["player1".to_owned()],
            word: "hoge1".to_owned(),
        }
    )]
    #[test_case(
        domain::CitizenGroup::new(
            vec![domain::Id::new("player2"), domain::Id::new("player3")],
            domain::Word::try_new("hoge2").unwrap(),
        )
        =>
        Group {
            players: vec!["player2".to_owned(), "player3".to_owned()],
            word: "hoge2".to_owned(),
        }
    )]
    fn group_from_citizen_group_domain_works(d: domain::CitizenGroup) -> Group {
        Group::from(d)
    }

    #[test_case(
        domain::Room::try_new(
            domain::Id::new("room1"),
            domain::PlayerCount::try_new(3).unwrap(),
            domain::WolfCount::try_new(1).unwrap(),
            domain::Id::new("player1"),
            vec![
                domain::Id::new("player1"),
                domain::Id::new("player2"),
                domain::Id::new("player3"),
            ],
            domain::GameMinutes::try_new(3).unwrap(),
            domain::ThemeKind::try_new("kind1").unwrap(),
        ).unwrap()
        =>
        Room {
            id: "room1".to_owned(),
            player_count: 3,
            wolf_count: 1,
            host_player_id: "player1".to_owned(),
            all_players: vec![
                "player1".to_owned(),
                "player2".to_owned(),
                "player3".to_owned(),
            ],
            game_time: chrono::Duration::minutes(3),
            theme_kind: "kind1".to_owned(),
        }
    )]
    #[test_case(
        domain::Room::try_new(
            domain::Id::new("room2"),
            domain::PlayerCount::try_new(5).unwrap(),
            domain::WolfCount::try_new(2).unwrap(),
            domain::Id::new("player11"),
            vec![
                domain::Id::new("player11"),
                domain::Id::new("player12"),
                domain::Id::new("player13"),
                domain::Id::new("player14"),
                domain::Id::new("player15"),
            ],
            domain::GameMinutes::try_new(5).unwrap(),
            domain::ThemeKind::try_new("kind2").unwrap(),
        ).unwrap()
        =>
        Room {
            id: "room2".to_owned(),
            player_count: 5,
            wolf_count: 2,
            host_player_id: "player11".to_owned(),
            all_players: vec![
                "player11".to_owned(),
                "player12".to_owned(),
                "player13".to_owned(),
                "player14".to_owned(),
                "player15".to_owned(),
            ],
            game_time: chrono::Duration::minutes(5),
            theme_kind: "kind2".to_owned(),
        }
    )]
    fn room_from_domain_works(d: domain::Room) -> Room {
        Room::from(d)
    }

    fn datetime(year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> DateTime<Tz> {
        chrono_tz::Japan
            .ymd(year, month, day)
            .and_hms(hour, min, sec)
    }
}
