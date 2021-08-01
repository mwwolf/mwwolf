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
}

#[derive(new, Getters, Clone, Debug, PartialEq)]
pub struct Talk {
    id: Id<Talk>,
    theme_id: Id<Theme>,
    ended_at: DateTime<Tz>,
    players: Vec<Id<Player>>,
    wolves: Group,
    citizen: Group,
    talk_time: TalkTime,
}

impl Talk {
    pub fn try_new(
        id: Id<Self>,
        theme_id: Id<Theme>,
        ended_at: DateTime<Tz>,
        players: Vec<Id<Player>>,
        wolves: Group,
        citizen: Group,
        talk_time: TalkTime,
    ) -> DomainResult<Self> {
        let talk = Self {
            id,
            theme_id,
            ended_at,
            players,
            wolves,
            citizen,
            talk_time,
        };
        talk.validate()?;
        Ok(talk)
    }

    pub fn join(&mut self, id: Id<Player>) -> DomainResult<()> {
        let mut new_self = self.clone();
        // NOTE: Talkにプレイヤー数を持たせるか、Playsersの型を新たに作るか要検討.
        // または他のドメインモデルを検討
        new_self.players.push(id);
        new_self.validate()?;
        *self = new_self;
        Ok(())
    }

    pub fn start(self, _: Id<Player>) -> DomainResult<Talk> {
        todo!("wolvesとcitizenにプレイヤーを振り分ける")
    }

    fn validate(&self) -> DomainResult<()> {
        Ok(())
    }
}

pub trait TalkFactory {
    fn create(
        &self,
        theme_id: Id<Theme>,
        timelimit_min: Duration,
        player_count: usize,
        wolves: Group,
        citizen: Group,
    );
}

#[derive(new, Getters, Clone, Debug, PartialEq)]
pub struct Group {
    players: Vec<Id<Player>>,
    count: usize,
    word: Word,
}

impl Group {
    pub fn new_with_added(&self, id: Id<Player>) -> DomainResult<Group> {
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
        Id::new("thema_1"),
        datetime(2021, 7, 30, 21, 19, 40),
        vec![Id::new("player1"), Id::new("player2")],
        Group::new(vec![], 3, Word::try_new("Test").unwrap()),
        Group::new(vec![], 5, Word::try_new("Test2").unwrap()),
        TalkTime::try_minutes(5).unwrap()
     => Ok(Talk{
        id: Id::new("talk_1"),
        theme_id:  Id::new("thema_1"),
        ended_at:  datetime(2021, 7, 30, 21, 19, 40),
        players:  vec![Id::new("player1"), Id::new("player2")],
        wolves:   Group::new(vec![], 3, Word::try_new("Test").unwrap()),
        citizen:   Group::new(vec![], 5, Word::try_new("Test2").unwrap()),
        talk_time:  TalkTime::try_minutes(5).unwrap(),
    }))]
    fn talk_try_new_works(
        id: Id<Talk>,
        theme_id: Id<Theme>,
        ended_at: DateTime<Tz>,
        players: Vec<Id<Player>>,
        wolves: Group,
        citizen: Group,
        talk_time: TalkTime,
    ) -> DomainResult<Talk> {
        Talk::try_new(id, theme_id, ended_at, players, wolves, citizen, talk_time)
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
