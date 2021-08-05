use crate::*;
use chrono::*;
use chrono_tz::Tz;

#[derive(Clone, Debug, PartialEq)]
pub struct TalkTime(Duration);

impl TalkTime {
    const DEFAULT_MAX_LIMIT: i64 = 60;
    const DEFAULT_MIN_LIMIT: i64 = 1;
    pub fn try_minutes(talk_time: i64) -> DomainResult<TalkTime> {
        if !(Self::DEFAULT_MIN_LIMIT..=Self::DEFAULT_MAX_LIMIT).contains(&talk_time) {
            Err(DomainError::new(
                DomainErrorKind::InvalidInput,
                format!(
                    "{} is outside of limits. the range are min:{} ~ max:{}",
                    talk_time,
                    Self::DEFAULT_MIN_LIMIT,
                    Self::DEFAULT_MAX_LIMIT
                ),
            ))
        } else {
            Ok(TalkTime(Duration::minutes(talk_time)))
        }
    }

    pub fn ended_at(&self) -> DateTime<Tz> {
        todo!()
    }
}

#[derive(new, Getters, Clone, Debug, PartialEq)]
pub struct Talk {
    id: Id<Talk>,
    room_id: Id<Room>,
    theme_id: Id<Theme>,
    ended_at: DateTime<Tz>,
    wolves: WolfGroup,
    citizen: CitizenGroup,
}

impl Talk {
    pub fn try_new(
        id: Id<Self>,
        room_id: Id<Room>,
        theme_id: Id<Theme>,
        ended_at: DateTime<Tz>,
        wolves: WolfGroup,
        citizen: CitizenGroup,
    ) -> DomainResult<Self> {
        let talk = Self {
            id,
            room_id,
            theme_id,
            ended_at,
            wolves,
            citizen,
        };
        talk.validate()?;
        Ok(talk)
    }

    fn validate(&self) -> DomainResult<()> {
        Ok(())
    }
}

pub trait TalkFactory {
    fn create(
        &self,
        theme_id: Id<Theme>,
        room_id: Id<Room>,
        ended_at: DateTime<Tz>,
        wolf_group: WolfGroup,
        citizen_group: CitizenGroup,
    ) -> DomainResult<Talk>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct WolfGroup(Group);

impl WolfGroup {
    pub fn new(players: Vec<Id<Player>>, count: usize, word: Word) -> Self {
        Self(Group::new(players, count, word))
    }

    pub fn new_with_added(&self, id: Id<Player>) -> DomainResult<Self> {
        Ok(Self(self.0.new_with_added(id)?))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CitizenGroup(Group);

impl CitizenGroup {
    pub fn new(players: Vec<Id<Player>>, count: usize, word: Word) -> Self {
        Self(Group::new(players, count, word))
    }

    pub fn new_with_added(&self, id: Id<Player>) -> DomainResult<Self> {
        Ok(Self(self.0.new_with_added(id)?))
    }
}

#[derive(new, Getters, Clone, Debug, PartialEq)]
struct Group {
    players: Vec<Id<Player>>,
    count: usize,
    word: Word,
}

impl Group {
    fn new_with_added(&self, id: Id<Player>) -> DomainResult<Group> {
        let mut new_group = self.clone();
        new_group.players.push(id);
        Ok(new_group)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    fn datetime(year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> DateTime<Tz> {
        chrono_tz::Japan
            .ymd(year, month, day)
            .and_hms(hour, min, sec)
    }

    #[test_case(
        Id::new("talk_1"),
        Id::new("room_1"),
        Id::new("thema_1"),
        datetime(2021, 7, 30, 21, 19, 40),
        WolfGroup::new(vec![], 3, Word::try_new("Test").unwrap()),
        CitizenGroup::new(vec![], 5, Word::try_new("Test2").unwrap())
     => Ok(Talk{
        id: Id::new("talk_1"),
        room_id:Id::new("room_1"),
        theme_id:  Id::new("thema_1"),
        ended_at:  datetime(2021, 7, 30, 21, 19, 40),
        wolves:   WolfGroup::new(vec![], 3, Word::try_new("Test").unwrap()),
        citizen:   CitizenGroup::new(vec![], 5, Word::try_new("Test2").unwrap()),
    }))]
    fn talk_try_new_works(
        id: Id<Talk>,
        room_id: Id<Room>,
        theme_id: Id<Theme>,
        ended_at: DateTime<Tz>,
        wolves: WolfGroup,
        citizen: CitizenGroup,
    ) -> DomainResult<Talk> {
        Talk::try_new(id, room_id, theme_id, ended_at, wolves, citizen)
    }

    #[test_case(1 => Ok(TalkTime(Duration::minutes(1))))]
    #[test_case(60 => Ok(TalkTime(Duration::minutes(60))))]
    #[test_case(0 => Err(DomainError::new(
                DomainErrorKind::InvalidInput,
                "0 is outside of limits. the range are min:1 ~ max:60",
            )))]
    #[test_case(61 => Err(DomainError::new(
                DomainErrorKind::InvalidInput,
                "61 is outside of limits. the range are min:1 ~ max:60",
            )))]
    fn talk_time_try_minutes_works(minutes: i64) -> DomainResult<TalkTime> {
        TalkTime::try_minutes(minutes)
    }
}
