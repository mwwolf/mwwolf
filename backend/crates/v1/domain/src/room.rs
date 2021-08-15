use crate::*;
use rand::prelude::*;
use std::cell::RefCell;
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

#[derive(Getters, new, Clone, Debug, PartialEq)]
pub struct Room {
    id: Id<Self>,
    player_count: PlayerCount,
    wolf_count: WolfCount,
    host_player_id: Id<Player>,
    all_players: Vec<Id<Player>>,
    talk_time: TalkTime,
    theme_kind: ThemeKind,
}

impl Room {
    pub fn try_new(
        id: Id<Self>,
        player_count: PlayerCount,
        wolf_count: WolfCount,
        host_player_id: Id<Player>,
        all_players: Vec<Id<Player>>,
        talk_time: TalkTime,
        theme_kind: ThemeKind,
    ) -> DomainResult<Self> {
        let room = Room {
            id,
            player_count,
            wolf_count,
            host_player_id,
            all_players,
            talk_time,
            theme_kind,
        };
        room.validate()?;
        Ok(room)
    }

    fn validate(&self) -> DomainResult<()> {
        if self.player_count() <= self.wolf_count() {
            Err(DomainError::new(
                DomainErrorKind::InvalidInput,
                "player_count must be bigger than wold count",
            ))
        } else {
            Ok(())
        }
    }
}

pub trait RoomServiceTypeParameters {
    type TalkFactory: TalkFactory;
    type ThemeRepository: ThemeRepository;
    type DateTimeGen: time::DateTimeGen;
    type RngCore: rand::RngCore;
}

#[derive(new)]
pub struct RoomService<RST: RoomServiceTypeParameters> {
    talk_factory: RST::TalkFactory,
    theme_repository: RST::ThemeRepository,
    date_time_gen: RST::DateTimeGen,
    rng_core: RefCell<RST::RngCore>,
}

impl<RST: RoomServiceTypeParameters> RoomService<RST> {
    pub async fn start_talk(&self, room: &Room) -> DomainResult<Talk> {
        match self.theme_repository.find_by_kind(room.theme_kind()).await {
            Ok(themes) => {
                let theme = themes.choose(&mut *self.rng_core.borrow_mut()).unwrap();
                let mut all_players = room.all_players().clone();
                all_players.shuffle(&mut *self.rng_core.borrow_mut());
                let wolfs = all_players
                    .drain(0..*room.wolf_count().raw_count())
                    .collect::<Vec<Id<Player>>>();
                let citizen = all_players;
                let (wolf_word, citizen_word) = theme.choice_word(&mut *self.rng_core.borrow_mut());
                let wolf_group = WolfGroup::new(wolfs, wolf_word.clone());
                let citizen_group = CitizenGroup::new(citizen, citizen_word.clone());
                let ended_at = room.talk_time().calc_ended_at(&self.date_time_gen.now());

                self.talk_factory
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::*;
    use chrono_tz::*;
    use rand::rngs::mock::StepRng;
    use testmww::mock::mock_libmww::time;

    fn datetime(year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> DateTime<Tz> {
        chrono_tz::Japan
            .ymd(year, month, day)
            .and_hms(hour, min, sec)
    }

    struct MockRoomServiceTypeParameter {}

    impl RoomServiceTypeParameters for MockRoomServiceTypeParameter {
        type ThemeRepository = MockThemeRepository;
        type TalkFactory = MockTalkFactory;
        type DateTimeGen = time::MockDateTimeGen;
        type RngCore = rand::rngs::mock::StepRng;
    }

    #[test_case(
        Room::new(
            Id::new("room1"),
            PlayerCount::try_new(5).unwrap(),
            WolfCount::try_new(2).unwrap(),
            Id::new("player1"),
            vec![Id::new("player1"), Id::new("player2"),Id::new("player3"),Id::new("player4"),Id::new("player5")],
            TalkTime::try_new(5).unwrap(),
            ThemeKind::try_new("theme_kind1").unwrap(),
        ),
        datetime(2021, 8, 11, 12, 30, 15)
        =>
        Ok(
            Talk::try_new(
                Id::new("talk1"),
                Id::new("room1"),
                Id::new("theme1"),
                datetime(2021, 8, 11, 12, 35, 15),
                WolfGroup::new(vec![Id::new("player2"),Id::new("player3")], Word::try_new("foo").unwrap()),
                CitizenGroup::new(vec![Id::new("player4"),Id::new("player5"),Id::new("player1")], Word::try_new("hoge").unwrap()),
            ).unwrap()
        ) ; "max_players_is_5_and_given_2players"
    )]
    #[async_std::test]
    async fn room_start_talk_works(room: Room, now: DateTime<Tz>) -> DomainResult<Talk> {
        let mut mock_date_time_gen = time::MockDateTimeGen::new();
        mock_date_time_gen.expect_now().returning(move || now);

        let mut mock_talk_factory = MockTalkFactory::new();
        mock_talk_factory.expect_create().returning(
            |room_id, theme_id, ended_at, wolf_group, citizen_group| {
                Ok(Talk::new(
                    Id::new("talk1"),
                    room_id,
                    theme_id,
                    ended_at,
                    wolf_group,
                    citizen_group,
                ))
            },
        );

        let mut mock_theme_repository = MockThemeRepository::new();
        mock_theme_repository
            .expect_find_by_kind()
            .with(eq(room.theme_kind.clone()))
            .returning(|kind| {
                Ok(vec![Theme::new(
                    Id::new("theme1"),
                    kind.clone(),
                    Word::try_new("hoge").unwrap(),
                    Word::try_new("foo").unwrap(),
                )])
            });

        let room_service = RoomService::<MockRoomServiceTypeParameter>::new(
            mock_talk_factory,
            mock_theme_repository,
            mock_date_time_gen,
            RefCell::new(StepRng::new(0, 1)),
        );
        room_service.start_talk(&room).await
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
        TalkTime::try_new(3).unwrap(),
        ThemeKind::try_new("theme1").unwrap()
        =>
        Ok(Room {
            id: Id::new("romm1"),
            player_count: PlayerCount::try_new(5).unwrap(),
            wolf_count: WolfCount::try_new(4).unwrap(),
            host_player_id: Id::new("player1"),
            all_players: vec![],
            talk_time: TalkTime::try_new(3).unwrap(),
            theme_kind: ThemeKind::try_new("theme1").unwrap(),
        })
    )]
    #[test_case(
        Id::new("romm2"),
        PlayerCount::try_new(6).unwrap(),
        WolfCount::try_new(5).unwrap(),
        Id::new("player2"),
        vec![],
        TalkTime::try_new(4).unwrap(),
        ThemeKind::try_new("theme2").unwrap()
        =>
        Ok(Room {
            id: Id::new("romm2"),
            player_count: PlayerCount::try_new(6).unwrap(),
            wolf_count: WolfCount::try_new(5).unwrap(),
            host_player_id: Id::new("player2"),
            all_players: vec![],
            talk_time: TalkTime::try_new(4).unwrap(),
            theme_kind: ThemeKind::try_new("theme2").unwrap(),
        })
    )]
    #[test_case(
        Id::new("romm2"),
        PlayerCount::try_new(5).unwrap(),
        WolfCount::try_new(5).unwrap(),
        Id::new("player2"),
        vec![],
        TalkTime::try_new(4).unwrap(),
        ThemeKind::try_new("theme2").unwrap()
        =>
        Err(DomainError::new(DomainErrorKind::InvalidInput, "player_count must be bigger than wold count"))
    )]
    fn room_try_new_works(
        id: Id<Room>,
        player_count: PlayerCount,
        wolf_count: WolfCount,
        host_player_id: Id<Player>,
        all_players: Vec<Id<Player>>,
        talk_time: TalkTime,
        theme_kind: ThemeKind,
    ) -> DomainResult<Room> {
        Room::try_new(
            id,
            player_count,
            wolf_count,
            host_player_id,
            all_players,
            talk_time,
            theme_kind,
        )
    }
}
