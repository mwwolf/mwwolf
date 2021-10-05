use super::*;
use rand::prelude::*;
use time::DateTimeGen;

#[derive(Clone, Debug, PartialEq, Getters)]
pub struct PlayerCount {
    raw_player_count: usize,
}

impl PlayerCount {
    pub fn try_new(raw_player_count: usize) -> DomainResult<Self> {
        if raw_player_count == 0 {
            Err(DomainError::new(
                DomainErrorKind::InvalidInput,
                "raw_player_count should not be zero",
            ))
        } else {
            Ok(Self { raw_player_count })
        }
    }
}

impl PartialOrd<WolfCount> for PlayerCount {
    fn partial_cmp(&self, other: &WolfCount) -> Option<std::cmp::Ordering> {
        self.raw_player_count().partial_cmp(other.raw_count())
    }
}

impl PartialEq<WolfCount> for PlayerCount {
    fn eq(&self, other: &WolfCount) -> bool {
        self.raw_player_count() == other.raw_count()
    }
}

#[derive(Clone, Debug, PartialEq, Getters)]
pub struct WolfCount {
    raw_count: usize,
}

impl WolfCount {
    pub fn try_new(raw_count: usize) -> DomainResult<Self> {
        if raw_count == 0 {
            Err(DomainError::new(
                DomainErrorKind::InvalidInput,
                "raw_count should not be zero",
            ))
        } else {
            Ok(Self { raw_count })
        }
    }
}

#[derive(Getters, Clone, Debug, PartialEq)]
pub struct Room {
    id: Id<Self>,
    player_count: PlayerCount,
    wolf_count: WolfCount,
    host_player_id: Id<Player>,
    all_players: Vec<Id<Player>>,
    game_time: GameMinutes,
    theme_kind: ThemeKind,
}

impl Room {
    pub fn try_new(
        id: Id<Self>,
        player_count: PlayerCount,
        wolf_count: WolfCount,
        host_player_id: Id<Player>,
        all_players: Vec<Id<Player>>,
        game_time: GameMinutes,
        theme_kind: ThemeKind,
    ) -> DomainResult<Self> {
        let room = Room {
            id,
            player_count,
            wolf_count,
            host_player_id,
            all_players,
            game_time,
            theme_kind,
        };
        room.validate()?;
        Ok(room)
    }

    pub fn join_player(&mut self, player_id: Id<Player>) -> DomainResult<()> {
        let mut new_room = self.clone();
        new_room.all_players.push(player_id);
        new_room.all_players.sort();
        new_room.validate()?;
        *self = new_room;
        Ok(())
    }

    pub fn leave_player(&mut self, player_id: &Id<Player>) -> DomainResult<()> {
        if &self.host_player_id == player_id {
            Err(DomainError::new(
                DomainErrorKind::InvalidInput,
                format!("player_id:{} is host", player_id),
            ))
        } else if let Some(index) = self.all_players.iter().position(|id| id == player_id) {
            let mut new_room = self.clone();
            new_room.all_players.remove(index);
            new_room.validate()?;
            *self = new_room;
            Ok(())
        } else {
            Err(DomainError::new(
                DomainErrorKind::InvalidInput,
                format!("not exists player_id:{}", player_id),
            ))
        }
    }

    fn validate(&self) -> DomainResult<()> {
        if self.player_count() <= self.wolf_count() {
            Err(DomainError::new(
                DomainErrorKind::InvalidInput,
                "player_count must be bigger than wolf count",
            ))
        } else if &self.all_players().len() > self.player_count().raw_player_count() {
            Err(DomainError::new(
                DomainErrorKind::InvalidInput,
                format!(
                    "player count is begger than max player count. current player count is {}, max player count is {}",
                    self.all_players().len(),
                    self.player_count().raw_player_count(),
                )))
        } else if self.has_duplicate_players() {
            Err(DomainError::new(
                DomainErrorKind::InvalidInput,
                "all_players must not be duplicate",
            ))
        } else {
            Ok(())
        }
    }
    fn has_duplicate_players(&self) -> bool {
        let mut before = None;
        for player_id in self.all_players().iter() {
            if let Some(id) = before {
                if id == player_id {
                    return true;
                }
            }
            before = Some(player_id);
        }
        false
    }
}

pub trait RoomServiceTypeParameters {
    type GameFactory: GameFactory;
    type ThemeRepository: ThemeRepository;
    type DateTimeGen: time::DateTimeGen;
    type RngFactory: RngFactory;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait RoomService: Send + Sync {
    async fn start_game(&self, room: &Room) -> DomainResult<Game>;
}

#[derive(new)]
pub struct RoomServiceImpl<RST: RoomServiceTypeParameters> {
    game_factory: RST::GameFactory,
    theme_repository: RST::ThemeRepository,
    date_time_gen: RST::DateTimeGen,
    rng_factory: RST::RngFactory,
}

#[async_trait]
impl<RST: RoomServiceTypeParameters> RoomService for RoomServiceImpl<RST> {
    async fn start_game(&self, room: &Room) -> DomainResult<Game> {
        match self.theme_repository.find_by_kind(room.theme_kind()).await {
            Ok(themes) => {
                let mut rng_core = self.rng_factory.create();
                let theme = themes.choose(&mut rng_core).ok_or_else(|| {
                    DomainError::new(
                        DomainErrorKind::Fail,
                        format!(
                            "themes of related of {:?} does not exists",
                            room.theme_kind()
                        ),
                    )
                })?;
                let mut all_players = room.all_players().clone();
                all_players.shuffle(&mut rng_core);
                let wolfs = all_players
                    .drain(0..*room.wolf_count().raw_count())
                    .collect::<Vec<Id<Player>>>();
                let citizen = all_players;
                let (wolf_word, citizen_word) = theme.choice_word(&mut rng_core);
                let wolf_group = WolfGroup::new(wolfs, wolf_word.clone());
                let citizen_group = CitizenGroup::new(citizen, citizen_word.clone());
                let ended_at = room.game_time().calc_ended_at(&self.date_time_gen.now());

                self.game_factory
                    .create(
                        room.id().clone(),
                        theme.id().clone(),
                        ended_at,
                        wolf_group,
                        citizen_group,
                    )
                    .await
            }
            Err(err) => Err(DomainError::new_with_source(
                DomainErrorKind::Fail,
                &format!(
                    "not found themes by search theme_kind:{:?}",
                    room.theme_kind()
                ),
                err.into(),
            )),
        }
    }
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait RoomRepository: Send + Sync {
    async fn find(&self, id: &Id<Room>) -> RepositoryResult<Room>;
    async fn store(&self, room: &Room) -> RepositoryResult<()>;
    async fn delete(&self, id: &Id<Room>) -> RepositoryResult<()>;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait RoomFactory: Send + Sync {
    async fn create(
        &self,
        player_count: PlayerCount,
        wolf_count: WolfCount,
        host_player_id: Id<Player>,
        game_time: GameMinutes,
        theme_kind: ThemeKind,
    ) -> DomainResult<Room>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testmww::mock::mock_libmww::time;
    use chrono::*;
    use chrono_tz::*;

    fn datetime(year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> DateTime<Tz> {
        chrono_tz::Japan
            .ymd(year, month, day)
            .and_hms(hour, min, sec)
    }

    struct MockRoomServiceTypeParameter {}

    impl RoomServiceTypeParameters for MockRoomServiceTypeParameter {
        type ThemeRepository = MockThemeRepository;
        type GameFactory = MockGameFactory;
        type DateTimeGen = time::MockDateTimeGen;
        type RngFactory = MockRngFactory;
    }

    #[test_case(
        Room::try_new(
            Id::new("room1"),
            PlayerCount::try_new(5).unwrap(),
            WolfCount::try_new(2).unwrap(),
            Id::new("player1"),
            vec![Id::new("player1"), Id::new("player2"),Id::new("player3"),Id::new("player4"),Id::new("player5")],
            GameMinutes::try_new(5).unwrap(),
            ThemeKind::try_new("theme_kind1").unwrap(),
        ).unwrap(),
        datetime(2021, 8, 11, 12, 30, 15),
        Id::new("game1"),
        Ok(
            vec![
                Theme::new(
                    Id::new("theme1"),
                    ThemeKind::try_new("theme_kind1").unwrap(),
                    Word::try_new("hoge").unwrap(),
                    Word::try_new("foo").unwrap(),
                ),
            ]
        ),
        MockRngFactory::new(0, 1)
        =>
        Ok(
            Game::try_new(
                Id::new("game1"),
                Id::new("room1"),
                Id::new("theme1"),
                datetime(2021, 8, 11, 12, 35, 15),
                WolfGroup::new(vec![Id::new("player2"),Id::new("player3")], Word::try_new("foo").unwrap()),
                CitizenGroup::new(vec![Id::new("player4"),Id::new("player5"),Id::new("player1")], Word::try_new("hoge").unwrap()),
                VoteBox::new(vec![]),
                GameStatus::Talking,
            ).unwrap()
        ) ; "max_players_is_5_and_given_2players"
    )]
    #[test_case(
        Room::try_new(
            Id::new("room2"),
            PlayerCount::try_new(6).unwrap(),
            WolfCount::try_new(3).unwrap(),
            Id::new("player2"),
            vec![Id::new("player2"), Id::new("player3"),Id::new("player4"),Id::new("player5"),Id::new("player6"), Id::new("player7")],
            GameMinutes::try_new(6).unwrap(),
            ThemeKind::try_new("theme_kind2").unwrap(),
        ).unwrap(),
        datetime(2022, 8, 11, 12, 30, 15),
        Id::new("game2"),
        Ok(
            vec![
                Theme::new(
                    Id::new("theme2"),
                    ThemeKind::try_new("theme_kind2").unwrap(),
                    Word::try_new("hoge2").unwrap(),
                    Word::try_new("foo2").unwrap(),
                ),
            ]
        ),
        MockRngFactory::new(0, 1)
        =>
        Ok(
            Game::try_new(
                Id::new("game2"),
                Id::new("room2"),
                Id::new("theme2"),
                datetime(2022, 8, 11, 12, 36, 15),
                WolfGroup::new(vec![Id::new("player3"),Id::new("player4"),Id::new("player5")], Word::try_new("foo2").unwrap()),
                CitizenGroup::new(vec![Id::new("player6"),Id::new("player7"),Id::new("player2")], Word::try_new("hoge2").unwrap()),
                VoteBox::new(vec![]),
                GameStatus::Talking,
            ).unwrap()
        ) ; "max_players_is_6_and_given_3players"
    )]
    #[test_case(
        Room::try_new(
            Id::new("room1"),
            PlayerCount::try_new(5).unwrap(),
            WolfCount::try_new(2).unwrap(),
            Id::new("player1"),
            vec![Id::new("player1"), Id::new("player2"),Id::new("player3"),Id::new("player4"),Id::new("player5")],
            GameMinutes::try_new(5).unwrap(),
            ThemeKind::try_new("theme_kind1").unwrap(),
        ).unwrap(),
        datetime(2021, 8, 11, 12, 30, 15),
        Id::new("game1"),
        Ok(
            vec![
                Theme::new(
                    Id::new("theme1"),
                    ThemeKind::try_new("theme_kind1").unwrap(),
                    Word::try_new("hoge").unwrap(),
                    Word::try_new("foo").unwrap(),
                ),
                Theme::new(
                    Id::new("theme2"),
                    ThemeKind::try_new("theme_kind2").unwrap(),
                    Word::try_new("hoge2").unwrap(),
                    Word::try_new("foo2").unwrap(),
                ),
            ]
        ),
        MockRngFactory::new(0, 1)
        =>
        Ok(
            Game::try_new(
                Id::new("game1"),
                Id::new("room1"),
                Id::new("theme1"),
                datetime(2021, 8, 11, 12, 35, 15),
                WolfGroup::new(vec![Id::new("player2"),Id::new("player3")], Word::try_new("foo").unwrap()),
                CitizenGroup::new(vec![Id::new("player4"),Id::new("player5"),Id::new("player1")], Word::try_new("hoge").unwrap()),
                VoteBox::new(vec![]),
                GameStatus::Talking,
            ).unwrap()
        ) ; "max_players_is_5_and_given_2players_return_multi_theme"
    )]
    #[test_case(
        Room::try_new(
            Id::new("room1"),
            PlayerCount::try_new(5).unwrap(),
            WolfCount::try_new(2).unwrap(),
            Id::new("player1"),
            vec![Id::new("player1"), Id::new("player2"),Id::new("player3"),Id::new("player4"),Id::new("player5")],
            GameMinutes::try_new(5).unwrap(),
            ThemeKind::try_new("theme_kind1").unwrap(),
        ).unwrap(),
        datetime(2021, 8, 11, 12, 30, 15),
        Id::new("game1"),
        Ok(vec![]),
        MockRngFactory::new(0, 1)
        =>
        Err(
            DomainError::new(DomainErrorKind::Fail, "themes of related of ThemeKind(\"theme_kind1\") does not exists")
        ) ; "fail_max_players_is_5_and_given_2players_return_zero_theme"
    )]
    #[async_std::test]
    async fn room_start_game_works(
        room: Room,
        now: DateTime<Tz>,
        new_game_id: Id<Game>,
        return_themes_result: RepositoryResult<Vec<Theme>>,
        rng_factory: MockRngFactory,
    ) -> DomainResult<Game> {
        let mut mock_date_time_gen = time::MockDateTimeGen::new();
        mock_date_time_gen.expect_now().returning(move || now);

        let mut mock_game_factory = MockGameFactory::new();
        mock_game_factory.expect_create().returning(
            move |room_id, theme_id, ended_at, wolf_group, citizen_group| {
                Ok(Game::new(
                    new_game_id.clone(),
                    room_id,
                    theme_id,
                    ended_at,
                    wolf_group,
                    citizen_group,
                    VoteBox::new(vec![]),
                    GameStatus::Talking,
                ))
            },
        );

        let mut mock_theme_repository = MockThemeRepository::new();
        mock_theme_repository
            .expect_find_by_kind()
            .with(eq(room.theme_kind.clone()))
            .returning(move |_| match &return_themes_result {
                Err(e) => Err(RepositoryError::new(e.kind().clone(), e.message())),
                Ok(ref v) => Ok(v.clone()),
            });

        let room_service = RoomServiceImpl::<MockRoomServiceTypeParameter>::new(
            mock_game_factory,
            mock_theme_repository,
            mock_date_time_gen,
            rng_factory,
        );
        room_service.start_game(&room).await
    }

    #[test_case(0 => Err(DomainError::new(DomainErrorKind::InvalidInput, "raw_count should not be zero")))]
    #[test_case(1 => Ok(WolfCount{ raw_count: 1 }))]
    #[test_case(100 => Ok(WolfCount{ raw_count: 100 }))]
    fn wolf_count_try_new_works(raw_count: usize) -> DomainResult<WolfCount> {
        WolfCount::try_new(raw_count)
    }

    #[test_case(0 => Err(DomainError::new(DomainErrorKind::InvalidInput, "raw_player_count should not be zero")))]
    #[test_case(1 => Ok(PlayerCount{ raw_player_count: 1 }))]
    #[test_case(100 => Ok(PlayerCount{ raw_player_count: 100 }))]
    fn player_count_try_new_works(raw_player_count: usize) -> DomainResult<PlayerCount> {
        PlayerCount::try_new(raw_player_count)
    }

    #[test_case(
        Id::new("romm1"),
        PlayerCount::try_new(5).unwrap(),
        WolfCount::try_new(4).unwrap(),
        Id::new("player1"),
        vec![],
        GameMinutes::try_new(3).unwrap(),
        ThemeKind::try_new("theme1").unwrap()
        =>
        Ok(Room {
            id: Id::new("romm1"),
            player_count: PlayerCount::try_new(5).unwrap(),
            wolf_count: WolfCount::try_new(4).unwrap(),
            host_player_id: Id::new("player1"),
            all_players: vec![],
            game_time: GameMinutes::try_new(3).unwrap(),
            theme_kind: ThemeKind::try_new("theme1").unwrap(),
        })
    )]
    #[test_case(
        Id::new("romm2"),
        PlayerCount::try_new(6).unwrap(),
        WolfCount::try_new(5).unwrap(),
        Id::new("player2"),
        vec![],
        GameMinutes::try_new(4).unwrap(),
        ThemeKind::try_new("theme2").unwrap()
        =>
        Ok(Room {
            id: Id::new("romm2"),
            player_count: PlayerCount::try_new(6).unwrap(),
            wolf_count: WolfCount::try_new(5).unwrap(),
            host_player_id: Id::new("player2"),
            all_players: vec![],
            game_time: GameMinutes::try_new(4).unwrap(),
            theme_kind: ThemeKind::try_new("theme2").unwrap(),
        })
    )]
    #[test_case(
        Id::new("romm2"),
        PlayerCount::try_new(5).unwrap(),
        WolfCount::try_new(5).unwrap(),
        Id::new("player2"),
        vec![],
        GameMinutes::try_new(4).unwrap(),
        ThemeKind::try_new("theme2").unwrap()
        =>
        Err(DomainError::new(DomainErrorKind::InvalidInput, "player_count must be bigger than wolf count"))
    )]
    fn room_try_new_works(
        id: Id<Room>,
        player_count: PlayerCount,
        wolf_count: WolfCount,
        host_player_id: Id<Player>,
        all_players: Vec<Id<Player>>,
        game_time: GameMinutes,
        theme_kind: ThemeKind,
    ) -> DomainResult<Room> {
        Room::try_new(
            id,
            player_count,
            wolf_count,
            host_player_id,
            all_players,
            game_time,
            theme_kind,
        )
    }

    #[test_case(
        Room{
            id: Id::new("room1"),
            player_count: PlayerCount::try_new(3).unwrap(),
            wolf_count: WolfCount::try_new(1).unwrap(),
            host_player_id: Id::new("player1"),
            all_players: vec![Id::new("player1"), Id::new("player2"), Id::new("player3")],
            game_time: GameMinutes::try_new(4).unwrap(),
            theme_kind: ThemeKind::try_new("theme1").unwrap(),
        } => Ok(()) ; "success"
    )]
    #[test_case(
        Room{
            id: Id::new("room1"),
            player_count: PlayerCount::try_new(3).unwrap(),
            wolf_count: WolfCount::try_new(3).unwrap(),
            host_player_id: Id::new("player1"),
            all_players: vec![Id::new("player1"), Id::new("player2"), Id::new("player3")],
            game_time: GameMinutes::try_new(4).unwrap(),
            theme_kind: ThemeKind::try_new("theme1").unwrap(),
        } => Err(DomainError::new(
                DomainErrorKind::InvalidInput,
                "player_count must be bigger than wolf count",
            )) ; "wolf_count >= player_count"
    )]
    #[test_case(
        Room{
            id: Id::new("room1"),
            player_count: PlayerCount::try_new(3).unwrap(),
            wolf_count: WolfCount::try_new(1).unwrap(),
            host_player_id: Id::new("player1"),
            all_players: vec![Id::new("player1"), Id::new("player2"), Id::new("player3"), Id::new("player4")],
            game_time: GameMinutes::try_new(4).unwrap(),
            theme_kind: ThemeKind::try_new("theme1").unwrap(),
        } => Err(DomainError::new(
                DomainErrorKind::InvalidInput,
                "player count is begger than max player count. current player count is 4, max player count is 3",
            )) ; "current player_count > max player_count"
    )]
    fn room_validate_works(room: Room) -> DomainResult<()> {
        room.validate()
    }

    #[test_case(
        Room{
            id: Id::new("room1"),
            player_count: PlayerCount::try_new(3).unwrap(),
            wolf_count: WolfCount::try_new(1).unwrap(),
            host_player_id: Id::new("player1"),
            all_players: vec![Id::new("player1")],
            game_time: GameMinutes::try_new(4).unwrap(),
            theme_kind: ThemeKind::try_new("theme1").unwrap(),
        },
        Id::new("player2"),
        &[Id::new("player1"), Id::new("player2")]
        => Ok(())
    )]
    #[test_case(
        Room{
            id: Id::new("room1"),
            player_count: PlayerCount::try_new(3).unwrap(),
            wolf_count: WolfCount::try_new(1).unwrap(),
            host_player_id: Id::new("player1"),
            all_players: vec![Id::new("player1"), Id::new("player2"), Id::new("player3")],
            game_time: GameMinutes::try_new(4).unwrap(),
            theme_kind: ThemeKind::try_new("theme1").unwrap(),
        },
        Id::new("player4"),
        &[Id::new("player1"), Id::new("player2"), Id::new("player3")]
        => Err(DomainError::new(
                DomainErrorKind::InvalidInput,
                "player count is begger than max player count. current player count is 4, max player count is 3",
            )) ; "current player_count > max player_count"
    )]
    #[test_case(
        Room{
            id: Id::new("room1"),
            player_count: PlayerCount::try_new(4).unwrap(),
            wolf_count: WolfCount::try_new(1).unwrap(),
            host_player_id: Id::new("player1"),
            all_players: vec![Id::new("player1"), Id::new("player2"), Id::new("player3")],
            game_time: GameMinutes::try_new(4).unwrap(),
            theme_kind: ThemeKind::try_new("theme1").unwrap(),
        },
        Id::new("player3"),
        &[Id::new("player1"), Id::new("player2"), Id::new("player3")]
        => Err(DomainError::new(
                DomainErrorKind::InvalidInput,
                "all_players must not be duplicate",
            )) ; "player is duplicate"
    )]
    fn room_join_player_works(
        mut room: Room,
        palyer_id: Id<Player>,
        expected_all_players: &[Id<Player>],
    ) -> DomainResult<()> {
        let result = room.join_player(palyer_id);
        assert_eq!(expected_all_players, room.all_players());
        result
    }

    #[test_case(
        Room{
            id: Id::new("room1"),
            player_count: PlayerCount::try_new(3).unwrap(),
            wolf_count: WolfCount::try_new(1).unwrap(),
            host_player_id: Id::new("player1"),
            all_players: vec![Id::new("player1"),Id::new("player2")],
            game_time: GameMinutes::try_new(4).unwrap(),
            theme_kind: ThemeKind::try_new("theme1").unwrap(),
        },
        Id::new("player2"),
        &[Id::new("player1")]
        => Ok(());"succeed leave"
    )]
    #[test_case(
        Room{
            id: Id::new("room1"),
            player_count: PlayerCount::try_new(3).unwrap(),
            wolf_count: WolfCount::try_new(1).unwrap(),
            host_player_id: Id::new("player1"),
            all_players: vec![Id::new("player1"),Id::new("player2")],
            game_time: GameMinutes::try_new(4).unwrap(),
            theme_kind: ThemeKind::try_new("theme1").unwrap(),
        },
        Id::new("player1"),
        &[Id::new("player1"),Id::new("player2")]
        => Err(DomainError::new(
                DomainErrorKind::InvalidInput,
                format!("player_id:{} is host", "player1"),
            ));"can not leave host"
    )]
    fn room_leave_player_works(
        mut room: Room,
        player_id: Id<Player>,
        expected_all_players: &[Id<Player>],
    ) -> DomainResult<()> {
        let result = room.leave_player(&player_id);
        assert_eq!(expected_all_players, room.all_players());
        result
    }
}
