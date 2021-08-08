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

pub struct RoomService<RST: RoomServiceTypeParameters> {
    talk_factory: RST::TalkFactory,
    theme_repository: RST::ThemeRepository,
    date_time_gen: RST::DateTimeGen,
}

impl<RST: RoomServiceTypeParameters> RoomService<RST> {
    pub fn start_talk(&self, room: &Room) -> DomainResult<Talk> {
        match self.theme_repository.find_by_kind(room.theme_kind()) {
            Ok(themes) => {
                let theme = themes.choose(&mut rand::thread_rng()).unwrap();
                let mut all_players = room.all_players().clone();
                let raw_wolf_count = *room.wolf_count().raw_count();
                let mut tr = rand::thread_rng();
                all_players.shuffle(&mut tr);
                let wolfs = all_players
                    .drain(0..raw_wolf_count)
                    .collect::<Vec<Id<Player>>>();
                let citizen = all_players;
                let (wolf_word, citizen_word) = theme.choice_word();
                let wolf_group = WolfGroup::new(wolfs, raw_wolf_count, wolf_word.clone());
                let citizen_group = CitizenGroup::new(
                    citizen,
                    room.all_players().len() - raw_wolf_count,
                    citizen_word.clone(),
                );
                let ended_at = room.talk_time().calc_ended_at(&self.date_time_gen.now());

                self.talk_factory.create(
                    theme.id().clone(),
                    room.id().clone(),
                    ended_at,
                    wolf_group,
                    citizen_group,
                )
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
