use crate::*;
use chrono::*;
use chrono_tz::Tz;

#[derive(new, Getters, Clone, Debug, PartialEq)]
pub struct Talk {
    id: Id<Talk>,
    theme_id: Id<Theme>,
    ended_at: DateTime<Tz>,
    players: Vec<Id<Player>>,
    wolves: Group,
    citizen: Group,
    timelimit_min: Duration,
}

impl Talk {
    pub fn try_new(
        id: Id<Self>,
        theme_id: Id<Theme>,
        ended_at: DateTime<Tz>,
        players: Vec<Id<Player>>,
        wolves: Group,
        citizen: Group,
        timelimit_min: Duration, // TODO(ryutah): VOにする
    ) -> DomainResult<Self> {
        let talk = Self {
            id,
            theme_id,
            ended_at,
            players,
            wolves,
            citizen,
            timelimit_min,
        };
        talk.validate()?;
        Ok(talk)
    }

    pub fn join(&mut self, id: Id<Player>) -> DomainResult<()> {
        let mut new_self = self.clone();
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
    pub fn added(&self, _: Id<Player>) -> DomainResult<Group> {
        todo!("")
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
        Duration::minutes(5)
     => Ok(Talk{
        id: Id::new("talk_1"),
        theme_id:  Id::new("thema_1"),
        ended_at:  datetime(2021, 7, 30, 21, 19, 40),
        players:  vec![Id::new("player1"), Id::new("player2")],
        wolves:   Group::new(vec![], 3, Word::try_new("Test").unwrap()),
        citizen:   Group::new(vec![], 5, Word::try_new("Test2").unwrap()),
        timelimit_min:  Duration::minutes(5)
    }))]
    fn try_new_works(
        id: Id<Talk>,
        theme_id: Id<Theme>,
        ended_at: DateTime<Tz>,
        players: Vec<Id<Player>>,
        wolves: Group,
        citizen: Group,
        timelimit_min: Duration,
    ) -> DomainResult<Talk> {
        Talk::try_new(
            id,
            theme_id,
            ended_at,
            players,
            wolves,
            citizen,
            timelimit_min,
        )
    }
}
