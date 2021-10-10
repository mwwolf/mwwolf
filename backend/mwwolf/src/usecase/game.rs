use super::*;
use domain::GameRepository;

#[async_trait]
trait Game {
    async fn start_vote(&self, game_id: &str) -> Result<dto::Game>;
    async fn end_vote(&self, game_id: &str) -> Result<dto::Game>;
    async fn vote(&self, command: command::Vote) -> Result<dto::Game>;
}

pub trait GameParameters {
    type GameRepository: domain::GameRepository;
}

#[derive(new)]
pub struct GameImpl<P: GameParameters> {
    game_repository: P::GameRepository,
}

#[async_trait]
impl<P: GameParameters> Game for GameImpl<P> {
    async fn start_vote(&self, game_id: &str) -> Result<dto::Game> {
        // TODO(ryutah): add tests
        self.update_status(game_id, domain::GameStatus::Voting)
            .await
    }

    async fn end_vote(&self, _game_id: &str) -> Result<dto::Game> {
        todo!()
    }

    async fn vote(&self, _command: command::Vote) -> Result<dto::Game> {
        todo!()
    }
}

impl<P: GameParameters> GameImpl<P> {
    async fn update_status(
        &self,
        game_id: &str,
        to_status: domain::GameStatus,
    ) -> Result<dto::Game> {
        let game_id = domain::Id::new(game_id);
        let mut game = self
            .game_repository
            .find(&game_id)
            .await
            .map_err(|e| match e.kind() {
                domain::RepositoryErrorKind::NotFound => domain::DomainError::new_with_source(
                    domain::DomainErrorKind::Notfound,
                    format!("game({:?}) is not found", &game_id),
                    e.into(),
                ),
                _ => domain::DomainError::new_with_source(
                    domain::DomainErrorKind::Fail,
                    format!("failed to find game({})", &game_id),
                    e.into(),
                ),
            })?;
        game.set_status(to_status)?;

        self.game_repository.store(&game).await.map_err(|e| {
            domain::DomainError::new_with_source(
                domain::DomainErrorKind::Fail,
                "failed to store game",
                e.into(),
            )
        })?;
        Ok(game.into())
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, TimeZone};
    use chrono_tz::Tz;
    use mockall::predicate;
    use test_case::test_case;

    use super::*;

    struct MockGameParameters;

    impl GameParameters for MockGameParameters {
        type GameRepository = domain::MockGameRepository;
    }

    #[test_case(
        "game1",
        domain::GameStatus::Voting,
        domain::Game::try_new(
            domain::Id::new("game1"),
            domain::Id::new("room1"),
            domain::Id::new("theme1"),
            datetime(2021, 10, 10, 12, 13, 14),
            domain::WolfGroup::new(
                vec![domain::Id::new("player1"), domain::Id::new("player2")],
                domain::Word::try_new("wolf_word").unwrap(),
            ),
            domain::CitizenGroup::new(
                vec![
                    domain::Id::new("player3"),
                    domain::Id::new("player4"),
                    domain::Id::new("player5"),
                ],
                domain::Word::try_new("citizen_word").unwrap(),
            ),
            domain::VoteBox::new(vec![]),
            domain::GameStatus::Talking,
        ).unwrap(),
        domain::Game::try_new(
            domain::Id::new("game1"),
            domain::Id::new("room1"),
            domain::Id::new("theme1"),
            datetime(2021, 10, 10, 12, 13, 14),
            domain::WolfGroup::new(
                vec![domain::Id::new("player1"), domain::Id::new("player2")],
                domain::Word::try_new("wolf_word").unwrap(),
            ),
            domain::CitizenGroup::new(
                vec![
                    domain::Id::new("player3"),
                    domain::Id::new("player4"),
                    domain::Id::new("player5"),
                ],
                domain::Word::try_new("citizen_word").unwrap(),
            ),
            domain::VoteBox::new(vec![]),
            domain::GameStatus::Voting,
        ).unwrap()
    =>
    Ok(dto::Game::new(
        "game1".into(),
        "room1".into(),
        "theme1".into(),
        datetime(2021, 10, 10, 12, 13, 14),
        dto::Group::new(vec!["player1".into(), "player2".into()], "wolf_word".into()),
        dto::Group::new(vec!["player3".into(), "player4".into(), "player5".into()], "citizen_word".into()),
        dto::VoteBox::new(vec![]),
        "Voting".into(),
    )); "update to votiong")]
    #[async_std::test]
    async fn update_status_works(
        game_id: &str,
        to_status: domain::GameStatus,
        return_game: domain::Game,
        expect_store_game: domain::Game,
    ) -> Result<dto::Game> {
        let mut mock_game_repository = domain::MockGameRepository::new();
        mock_game_repository
            .expect_find()
            .with(predicate::eq(domain::Id::new(game_id)))
            .returning(move |_| Ok(return_game.clone()));
        mock_game_repository
            .expect_store()
            .with(predicate::eq(expect_store_game))
            .returning(|_| Ok(()));

        let game_usecase = GameImpl::<MockGameParameters>::new(mock_game_repository);
        game_usecase.update_status(game_id, to_status).await
    }

    fn datetime(year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> DateTime<Tz> {
        chrono_tz::Japan
            .ymd(year, month, day)
            .and_hms(hour, min, sec)
    }
}
