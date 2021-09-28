use super::*;
use chrono::*;
use chrono_tz::Tz;

#[derive(Clone, Debug, PartialEq)]
pub struct GameMinutes(Duration);

impl GameMinutes {
    const DEFAULT_MAX_LIMIT: u32 = 60;
    const DEFAULT_MIN_LIMIT: u32 = 1;
    pub fn try_new(game_time_minutes: u32) -> DomainResult<GameMinutes> {
        if !(Self::DEFAULT_MIN_LIMIT..=Self::DEFAULT_MAX_LIMIT).contains(&game_time_minutes) {
            Err(DomainError::new(
                DomainErrorKind::InvalidInput,
                format!(
                    "{} is outside of limits. the range are min:{} ~ max:{}",
                    game_time_minutes,
                    Self::DEFAULT_MIN_LIMIT,
                    Self::DEFAULT_MAX_LIMIT
                ),
            ))
        } else {
            Ok(GameMinutes(Duration::minutes(game_time_minutes as i64)))
        }
    }

    pub fn calc_ended_at(&self, started_at: &DateTime<Tz>) -> DateTime<Tz> {
        *started_at + self.0
    }
    pub fn raw_minutes(&self) -> &Duration {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, EnumString, ToString)]
pub enum GameStatus {
    Talking,
    Voting,
    Ended,
}

#[derive(new, Getters, Clone, Debug, PartialEq)]
pub struct Game {
    id: Id<Game>,
    room_id: Id<Room>,
    theme_id: Id<Theme>,
    ended_at: DateTime<Tz>,
    wolves: WolfGroup,
    citizen: CitizenGroup,
    vote_box: VoteBox,
    status: GameStatus,
}

impl Game {
    pub fn try_new(
        id: Id<Self>,
        room_id: Id<Room>,
        theme_id: Id<Theme>,
        ended_at: DateTime<Tz>,
        wolves: WolfGroup,
        citizen: CitizenGroup,
        vote_box: VoteBox,
        status: GameStatus,
    ) -> DomainResult<Self> {
        let game = Self {
            id,
            room_id,
            theme_id,
            ended_at,
            wolves,
            citizen,
            vote_box,
            status,
        };
        game.validate()?;
        Ok(game)
    }

    pub fn vote(&mut self, vote: Vote) -> DomainResult<VoteResult> {
        if *self.status() != GameStatus::Voting {
            Err(DomainError::new(
                DomainErrorKind::Fail,
                "game status is not voting",
            ))
        } else {
            self.vote_box = self.vote_box.new_with_added(vote)?;
            Ok(VoteResult::new(
                self.all_player_count() == self.vote_box.votes.len(),
            ))
        }
    }

    fn all_player_count(&self) -> usize {
        self.wolves.players().len() + self.citizen.players().len()
    }

    fn validate(&self) -> DomainResult<()> {
        Ok(())
    }
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait GameFactory {
    async fn create(
        &self,
        room_id: Id<Room>,
        theme_id: Id<Theme>,
        ended_at: DateTime<Tz>,
        wolf_group: WolfGroup,
        citizen_group: CitizenGroup,
    ) -> DomainResult<Game>;
}

#[derive(new, Getters, Clone, Debug, PartialEq)]
pub struct WolfGroup {
    players: Vec<Id<Player>>,
    word: Word,
}

impl WolfGroup {
    pub fn new_with_added(&self, id: Id<Player>) -> DomainResult<Self> {
        let mut new_group = self.clone();
        new_group.players.push(id);
        Ok(new_group)
    }
}

#[derive(new, Getters, Clone, Debug, PartialEq)]
pub struct CitizenGroup {
    players: Vec<Id<Player>>,
    word: Word,
}

impl CitizenGroup {
    pub fn new_with_added(&self, id: Id<Player>) -> DomainResult<Self> {
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

impl VoteBox {
    fn new_with_added(&self, vote: Vote) -> DomainResult<VoteBox> {
        if self.votes.iter().any(|v| v.voter == vote.voter) {
            Err(DomainError::new(
                DomainErrorKind::InvalidInput,
                format!("already voted in voter. vote:{:?}", vote),
            ))
        } else {
            let mut new_votes = self.votes.clone();
            new_votes.push(vote);
            Ok(VoteBox { votes: new_votes })
        }
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
        Id::new("game_1"),
        Id::new("room_1"),
        Id::new("thema_1"),
        datetime(2021, 7, 30, 21, 19, 40),
        WolfGroup::new(vec![], Word::try_new("Test").unwrap()),
        CitizenGroup::new(vec![], Word::try_new("Test2").unwrap()),
        VoteBox::new(vec![]),
        GameStatus::Talking
     => Ok(Game{
        id: Id::new("game_1"),
        room_id:Id::new("room_1"),
        theme_id:  Id::new("thema_1"),
        ended_at:  datetime(2021, 7, 30, 21, 19, 40),
        wolves:   WolfGroup::new(vec![], Word::try_new("Test").unwrap()),
        citizen:   CitizenGroup::new(vec![], Word::try_new("Test2").unwrap()),
        vote_box: VoteBox::new(vec![]),
        status:GameStatus::Talking,
    }))]
    fn game_try_new_works(
        id: Id<Game>,
        room_id: Id<Room>,
        theme_id: Id<Theme>,
        ended_at: DateTime<Tz>,
        wolves: WolfGroup,
        citizen: CitizenGroup,
        vote_box: VoteBox,
        status: GameStatus,
    ) -> DomainResult<Game> {
        Game::try_new(
            id, room_id, theme_id, ended_at, wolves, citizen, vote_box, status,
        )
    }

    #[test_case(1 => Ok(GameMinutes(Duration::minutes(1))))]
    #[test_case(60 => Ok(GameMinutes(Duration::minutes(60))))]
    #[test_case(0 => Err(DomainError::new(
                DomainErrorKind::InvalidInput,
                "0 is outside of limits. the range are min:1 ~ max:60",
            )))]
    #[test_case(61 => Err(DomainError::new(
                DomainErrorKind::InvalidInput,
                "61 is outside of limits. the range are min:1 ~ max:60",
            )))]
    fn game_time_try_minutes_works(minutes: u32) -> DomainResult<GameMinutes> {
        GameMinutes::try_new(minutes)
    }

    #[test_case(
        GameMinutes::try_new(3).unwrap(),
        datetime(2021,3,4,2,30,0)
        => datetime(2021,3,4,2,33,0)
        )]
    #[test_case(
        GameMinutes::try_new(5).unwrap(),
        datetime(2021,3,4,2,0,0)
        => datetime(2021,3,4,2,5,0)
        )]
    fn game_time_calc_ended_at_works(
        game_time: GameMinutes,
        started_at: DateTime<Tz>,
    ) -> DateTime<Tz> {
        game_time.calc_ended_at(&started_at)
    }

    #[test_case(
        Game::new(
            Id::new("game1"),
            Id::new("room_id"),
            Id::new("theme"),
            datetime(2021, 3, 4, 3, 2, 1),
            WolfGroup::new(vec![Id::new("player1")], Word::try_new("word1").unwrap()),
            CitizenGroup::new(vec![Id::new("player2")], Word::try_new("word2").unwrap()),
            VoteBox::new(vec![]),
            GameStatus::Voting,
        ),
        Vote::new(Id::new("player1"),Id::new("player2")),
        VoteBox::new(vec![Vote::new(Id::new("player1"),Id::new("player2"))])
        => Ok(VoteResult::new(false));"succeed but not yet end"
        )]
    #[test_case(
        Game::new(
            Id::new("game1"),
            Id::new("room_id"),
            Id::new("theme"),
            datetime(2021, 3, 4, 3, 2, 1),
            WolfGroup::new(vec![Id::new("player1")], Word::try_new("word1").unwrap()),
            CitizenGroup::new(vec![Id::new("player2")], Word::try_new("word2").unwrap()),
            VoteBox::new(vec![Vote::new(Id::new("player2"),Id::new("player1"))]),
            GameStatus::Voting,
        ),
        Vote::new(Id::new("player1"),Id::new("player2")),
        VoteBox::new(vec![Vote::new(Id::new("player2"),Id::new("player1")),Vote::new(Id::new("player1"),Id::new("player2"))])
        => Ok(VoteResult::new(true));"succeed and end"
        )]
    #[test_case(
        Game::new(
            Id::new("game1"),
            Id::new("room_id"),
            Id::new("theme"),
            datetime(2021, 3, 4, 3, 2, 1),
            WolfGroup::new(vec![Id::new("player1")], Word::try_new("word1").unwrap()),
            CitizenGroup::new(vec![Id::new("player2")], Word::try_new("word2").unwrap()),
            VoteBox::new(vec![]),
            GameStatus::Ended,
        ),
        Vote::new(Id::new("player1"),Id::new("player2")),
        VoteBox::new(vec![])
        => Err(DomainError::new(DomainErrorKind::Fail, "game status is not voting"));"game status is not voting"
        )]
    #[test_case(
        Game::new(
            Id::new("game1"),
            Id::new("room_id"),
            Id::new("theme"),
            datetime(2021, 3, 4, 3, 2, 1),
            WolfGroup::new(vec![Id::new("player1")], Word::try_new("word1").unwrap()),
            CitizenGroup::new(vec![Id::new("player2")], Word::try_new("word2").unwrap()),
            VoteBox::new(vec![Vote::new(Id::new("player1"),Id::new("player2"))]),
            GameStatus::Voting,
        ),
        Vote::new(Id::new("player1"),Id::new("player2")),
        VoteBox::new(vec![Vote::new(Id::new("player1"),Id::new("player2"))])
        => Err(DomainError::new(
                DomainErrorKind::InvalidInput,
                format!("already voted in voter. vote:{:?}", Vote::new(Id::new("player1"),Id::new("player2"))),
            ));"already voted"
        )]
    fn game_vote_works(
        mut game: Game,
        vote: Vote,
        expected_vote_box: VoteBox,
    ) -> DomainResult<VoteResult> {
        let result = game.vote(vote);
        assert_eq!(expected_vote_box, *game.vote_box());
        result
    }
}
