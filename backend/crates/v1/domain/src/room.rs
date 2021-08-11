use crate::*;
use rand::prelude::*;
use time::DateTimeGen;

#[derive(Clone, Debug, PartialEq, new, Getters)]
pub struct MaxPlayerCount {
    raw_max_player_count: usize,
}
#[derive(Clone, Debug, PartialEq, new, Getters)]
pub struct WolfCount {
    raw_count: usize,
}

#[derive(Getters, new, Clone, Debug, PartialEq)]
pub struct Room {
    id: Id<Room>,
    max_player_count: MaxPlayerCount,
    wolf_count: WolfCount,
    host_player_id: Id<Player>,
    all_players: Vec<Id<Player>>,
    talk_time: TalkTime,
    theme_kind: ThemeKind,
}

pub trait RoomServiceTypeParameters {
    type TalkFactory: TalkFactory;
    type ThemeRepository: ThemeRepository;
    type DateTimeGen: time::DateTimeGen;
}

#[derive(new)]
pub struct RoomService<RST: RoomServiceTypeParameters> {
    talk_factory: RST::TalkFactory,
    theme_repository: RST::ThemeRepository,
    date_time_gen: RST::DateTimeGen,
    random_factory: random::RngCoreFactory,
}

impl<RST: RoomServiceTypeParameters> RoomService<RST> {
    pub async fn start_talk(&self, room: &Room) -> DomainResult<Talk> {
        match self.theme_repository.find_by_kind(room.theme_kind()).await {
            Ok(themes) => {
                let mut tr = self.random_factory.create();
                let theme = themes.choose(&mut tr).unwrap();
                let mut all_players = room.all_players().clone();
                all_players.shuffle(&mut tr);
                let wolfs = all_players
                    .drain(0..*room.wolf_count().raw_count())
                    .collect::<Vec<Id<Player>>>();
                let citizen = all_players;
                let (wolf_word, citizen_word) = theme.choice_word();
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
    }

    #[test_case(
        Room::new(
            Id::new("room1"),
            MaxPlayerCount::new(5),
            WolfCount::new(2),
            Id::new("player1"),
            vec![Id::new("player1"), Id::new("player2")],
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
                WolfGroup::new(vec![Id::new("player1")], Word::try_new("hoge").unwrap()),
                CitizenGroup::new(vec![Id::new("player2")], Word::try_new("foo").unwrap()),
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
            random::RngCoreFactory::new(1, 1),
        );
        room_service.start_talk(&room).await
    }
}
