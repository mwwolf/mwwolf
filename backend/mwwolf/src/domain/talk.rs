use super::*;
use chrono::*;
use chrono_tz::Tz;

#[derive(Clone, Debug, PartialEq)]
pub struct TalkMinutes(Duration);

impl TalkMinutes {
    const DEFAULT_MAX_LIMIT: u32 = 60;
    const DEFAULT_MIN_LIMIT: u32 = 1;
    pub fn try_new(talk_time_minutes: u32) -> DomainResult<TalkMinutes> {
        if !(Self::DEFAULT_MIN_LIMIT..=Self::DEFAULT_MAX_LIMIT).contains(&talk_time_minutes) {
            Err(DomainError::new(
                DomainErrorKind::InvalidInput,
                format!(
                    "{} is outside of limits. the range are min:{} ~ max:{}",
                    talk_time_minutes,
                    Self::DEFAULT_MIN_LIMIT,
                    Self::DEFAULT_MAX_LIMIT
                ),
            ))
        } else {
            Ok(TalkMinutes(Duration::minutes(talk_time_minutes as i64)))
        }
    }

    pub fn calc_ended_at(&self, started_at: &DateTime<Tz>) -> DateTime<Tz> {
        *started_at + self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TalkStatus {
    Started,
    Ended,
}

#[derive(new, Getters, Clone, Debug, PartialEq)]
pub struct Talk {
    id: Id<Talk>,
    room_id: Id<Room>,
    theme_id: Id<Theme>,
    ended_at: DateTime<Tz>,
    wolves: WolfGroup,
    citizen: CitizenGroup,
    vote_box: VoteBox,
    status: TalkStatus,
}

impl Talk {
    pub fn try_new(
        id: Id<Self>,
        room_id: Id<Room>,
        theme_id: Id<Theme>,
        ended_at: DateTime<Tz>,
        wolves: WolfGroup,
        citizen: CitizenGroup,
        vote_box: VoteBox,
        status: TalkStatus,
    ) -> DomainResult<Self> {
        let talk = Self {
            id,
            room_id,
            theme_id,
            ended_at,
            wolves,
            citizen,
            vote_box,
            status,
        };
        talk.validate()?;
        Ok(talk)
    }

    pub fn vote(&mut self, _: Vote) -> DomainResult<VoteResult> {
        todo!()
    }

    fn validate(&self) -> DomainResult<()> {
        Ok(())
    }
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait TalkFactory {
    async fn create(
        &self,
        room_id: Id<Room>,
        theme_id: Id<Theme>,
        ended_at: DateTime<Tz>,
        wolf_group: WolfGroup,
        citizen_group: CitizenGroup,
    ) -> DomainResult<Talk>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct WolfGroup(Group);

impl WolfGroup {
    pub fn new(players: Vec<Id<Player>>, word: Word) -> Self {
        Self(Group::new(players, word))
    }

    pub fn new_with_added(&self, id: Id<Player>) -> DomainResult<Self> {
        Ok(Self(self.0.new_with_added(id)?))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CitizenGroup(Group);

impl CitizenGroup {
    pub fn new(players: Vec<Id<Player>>, word: Word) -> Self {
        Self(Group::new(players, word))
    }

    pub fn new_with_added(&self, id: Id<Player>) -> DomainResult<Self> {
        Ok(Self(self.0.new_with_added(id)?))
    }
}

#[derive(new, Getters, Clone, Debug, PartialEq)]
struct Group {
    players: Vec<Id<Player>>,
    word: Word,
}

impl Group {
    fn new_with_added(&self, id: Id<Player>) -> DomainResult<Group> {
        let mut new_group = self.clone();
        new_group.players.push(id);
        Ok(new_group)
    }
}
#[derive(new, Getters, Clone, Debug, PartialEq)]
pub struct VoteResult {
    is_end: bool,
}

#[derive(new, Getters, Clone, Debug, PartialEq)]
pub struct VoteBox {
    votes: Vec<Vote>,
}

#[derive(new, Getters, Clone, Debug, PartialEq)]
pub struct Vote {
    target: Id<Player>,
    voter: Id<Player>,
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
        WolfGroup::new(vec![], Word::try_new("Test").unwrap()),
        CitizenGroup::new(vec![], Word::try_new("Test2").unwrap()),
        VoteBox::new(vec![]),
        TalkStatus::Started
     => Ok(Talk{
        id: Id::new("talk_1"),
        room_id:Id::new("room_1"),
        theme_id:  Id::new("thema_1"),
        ended_at:  datetime(2021, 7, 30, 21, 19, 40),
        wolves:   WolfGroup::new(vec![], Word::try_new("Test").unwrap()),
        citizen:   CitizenGroup::new(vec![], Word::try_new("Test2").unwrap()),
        vote_box: VoteBox::new(vec![]),
        status:TalkStatus::Started,
    }))]
    fn talk_try_new_works(
        id: Id<Talk>,
        room_id: Id<Room>,
        theme_id: Id<Theme>,
        ended_at: DateTime<Tz>,
        wolves: WolfGroup,
        citizen: CitizenGroup,
        vote_box: VoteBox,
        status: TalkStatus,
    ) -> DomainResult<Talk> {
        Talk::try_new(
            id, room_id, theme_id, ended_at, wolves, citizen, vote_box, status,
        )
    }

    #[test_case(1 => Ok(TalkMinutes(Duration::minutes(1))))]
    #[test_case(60 => Ok(TalkMinutes(Duration::minutes(60))))]
    #[test_case(0 => Err(DomainError::new(
                DomainErrorKind::InvalidInput,
                "0 is outside of limits. the range are min:1 ~ max:60",
            )))]
    #[test_case(61 => Err(DomainError::new(
                DomainErrorKind::InvalidInput,
                "61 is outside of limits. the range are min:1 ~ max:60",
            )))]
    fn talk_time_try_minutes_works(minutes: u32) -> DomainResult<TalkMinutes> {
        TalkMinutes::try_new(minutes)
    }

    #[test_case(
        TalkMinutes::try_new(3).unwrap(),
        datetime(2021,3,4,2,30,0)
        => datetime(2021,3,4,2,33,0)
        )]
    #[test_case(
        TalkMinutes::try_new(5).unwrap(),
        datetime(2021,3,4,2,0,0)
        => datetime(2021,3,4,2,5,0)
        )]
    fn talk_time_calc_ended_at_works(
        talk_time: TalkMinutes,
        started_at: DateTime<Tz>,
    ) -> DateTime<Tz> {
        talk_time.calc_ended_at(&started_at)
    }
}
