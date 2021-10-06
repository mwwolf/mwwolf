use super::*;

use async_graphql::futures_util::TryFutureExt;
use domain::RoomFactory;
use domain::RoomRepository;
use domain::RoomService;

#[async_trait]
trait Room {
    async fn create(&self, command: command::RoomCreate) -> Result<dto::Room>;
    async fn delete(&self, room_id: &str) -> Result<()>;
    async fn join(&self, command: command::RoomJoin) -> Result<dto::Room>;
    async fn leave(&self, command: command::RoomLeave) -> Result<dto::Room>;
    async fn start_game(&self, command: command::StartGame) -> Result<dto::Game>;
}

pub trait RoomTypeParameters {
    type Service: domain::RoomService;
    type Repository: domain::RoomRepository;
    type Factory: domain::RoomFactory;
}

#[derive(new, Getters)]
pub struct RoomImpl<RST: RoomTypeParameters> {
    service: RST::Service,
    repository: RST::Repository,
    factory: RST::Factory,
}

#[async_trait]
impl<RST: RoomTypeParameters> Room for RoomImpl<RST> {
    async fn create(&self, create_command: command::RoomCreate) -> Result<dto::Room> {
        let room = self
            .factory
            .create(
                domain::PlayerCount::try_new(*create_command.player_count())?,
                domain::WolfCount::try_new(*create_command.wolf_count())?,
                domain::Id::new(create_command.host_player_id()),
                domain::GameMinutes::try_new(*create_command.game_minutes())?,
                domain::ThemeKind::try_new(create_command.theme_kind())?,
            )
            .await?;

        self.repository.store(&room).await.map_err(|e| {
            domain::DomainError::new_with_source(
                domain::DomainErrorKind::Fail,
                "cannnot store room",
                e.into(),
            )
        })?;

        Ok(room.into())
    }
    async fn delete(&self, room_id: &str) -> Result<()> {
        let room_id = domain::Id::new(room_id);
        self.repository
            .delete(&room_id)
            .map_err(|e| match e.kind() {
                domain::RepositoryErrorKind::NotFound => domain::DomainError::new_with_source(
                    domain::DomainErrorKind::Notfound,
                    format!("room(id {}) is not found", room_id),
                    e.into(),
                ),
                _ => domain::DomainError::new_with_source(
                    domain::DomainErrorKind::Fail,
                    format!("failed to delete room(id {})", room_id),
                    e.into(),
                ),
            })
            .await?;
        Ok(())
    }
    async fn join(&self, join_command: command::RoomJoin) -> Result<dto::Room> {
        let room_id = domain::Id::new(join_command.room_id());
        let mut room = self
            .repository
            .find(&room_id)
            .map_err(|e| match e.kind() {
                domain::RepositoryErrorKind::NotFound => domain::DomainError::new_with_source(
                    domain::DomainErrorKind::Notfound,
                    format!("room(id {}) is not found", room_id),
                    e.into(),
                ),
                _ => domain::DomainError::new_with_source(
                    domain::DomainErrorKind::Fail,
                    format!("failed to delete room(id {})", room_id),
                    e.into(),
                ),
            })
            .await?;

        let new_player = domain::Id::new(join_command.player_id());
        room.join_player(new_player)?;

        self.repository
            .store(&room)
            .map_err(|e| {
                domain::DomainError::new_with_source(
                    domain::DomainErrorKind::Fail,
                    format!("failed to store room: {:?}", room),
                    e.into(),
                )
            })
            .await?;
        Ok(room.into())
    }
    async fn leave(&self, command: command::RoomLeave) -> Result<dto::Room> {
        let room_id = domain::Id::new(command.room_id());
        let mut room = self
            .repository
            .find(&room_id)
            .await
            .map_err(|e| match e.kind() {
                domain::RepositoryErrorKind::NotFound => domain::DomainError::new_with_source(
                    domain::DomainErrorKind::Notfound,
                    format!("room(id {}) is not found", room_id),
                    e.into(),
                ),
                _ => domain::DomainError::new_with_source(
                    domain::DomainErrorKind::Fail,
                    format!("failed find room(id {})", room_id),
                    e.into(),
                ),
            })?;
        let leave_plaeyr_id = domain::Id::new(command.player_id());
        room.leave_player(&leave_plaeyr_id)?;

        self.repository.store(&room).await.map_err(|e| {
            domain::DomainError::new_with_source(
                domain::DomainErrorKind::Fail,
                format!("failed store room : {:?}", room),
                e.into(),
            )
        })?;
        Ok(room.into())
    }
    async fn start_game(&self, command: command::StartGame) -> Result<dto::Game> {
        let room_id = domain::Id::new(command.room_id());
        let room = self
            .repository
            .find(&room_id)
            .await
            .map_err(|e| match e.kind() {
                domain::RepositoryErrorKind::NotFound => domain::DomainError::new_with_source(
                    domain::DomainErrorKind::Notfound,
                    format!("room(id {}) is not found", room_id),
                    e.into(),
                ),
                _ => domain::DomainError::new_with_source(
                    domain::DomainErrorKind::Fail,
                    format!("failed find room(id {})", room_id),
                    e.into(),
                ),
            })?;
        let game = self.service.start_game(&room).await?;

        Ok(game.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::DateTime;
    use chrono::TimeZone;
    use chrono_tz::Tz;
    use mockall::*;
    use test_case::test_case;

    struct MockParameters;

    impl RoomTypeParameters for MockParameters {
        type Service = domain::MockRoomService;
        type Repository = domain::MockRoomRepository;
        type Factory = domain::MockRoomFactory;
    }
    #[test_case(
        command::RoomCreate::new(
            4,
            1,
            "hoge".into(),
            3,"foo".into()
        )
        =>
        Ok(dto::Room::new(
            "room1".into(),
            4,
            1,
            "hoge".into(),
            vec![],3,"foo".into(),
        ))
    )]
    #[test_case(
        command::RoomCreate::new(3,3,"hoge".into(),3,"foo".into())
        =>
        Err(domain::DomainError::new(domain::DomainErrorKind::InvalidInput,"player_count must be bigger than wolf count"))
    )]
    #[async_std::test]
    async fn create_room_impl_ok_works(create_command: command::RoomCreate) -> Result<dto::Room> {
        let mock_room_service = domain::MockRoomService::new();
        let mut mock_room_repository = domain::MockRoomRepository::new();
        let mut mock_room_factory = domain::MockRoomFactory::new();
        mock_room_factory.expect_create().returning(
            move |player_count, wolf_count, host_player_id, game_time, theme_kind| {
                domain::Room::try_new(
                    domain::Id::new("room1"),
                    player_count,
                    wolf_count,
                    host_player_id,
                    vec![],
                    game_time,
                    theme_kind,
                )
            },
        );
        mock_room_repository
            .expect_store()
            .returning(move |_| Ok(()));
        let room_usecase = RoomImpl::<MockParameters>::new(
            mock_room_service,
            mock_room_repository,
            mock_room_factory,
        );
        room_usecase.create(create_command).await
    }

    #[test_case(
        "room1", domain::Id::new("room1") => Ok(())
    )]
    #[async_std::test]
    async fn delete_room_impl_ok_works(
        id: &str,
        expect_room_id: domain::Id<domain::Room>,
    ) -> Result<()> {
        let mock_room_service = domain::MockRoomService::new();
        let mut mock_room_repository = domain::MockRoomRepository::new();
        let mock_room_factory = domain::MockRoomFactory::new();
        mock_room_repository
            .expect_delete()
            .with(predicate::eq(expect_room_id))
            .returning(|_| Ok(()));

        let room_usecase = RoomImpl::<MockParameters>::new(
            mock_room_service,
            mock_room_repository,
            mock_room_factory,
        );
        room_usecase.delete(id).await
    }

    #[test_case(
        command::RoomJoin::new("room1".into(), "new_player1".into()),
        domain::Id::new("room1"),
        domain::Room::try_new(
            domain::Id::new("room1"),
            domain::PlayerCount::try_new(3).unwrap(),
            domain::WolfCount::try_new(1).unwrap(),
            domain::Id::new("host1"),
            vec![domain::Id::new("player1")],
            domain::GameMinutes::try_new(3).unwrap(),
            domain::ThemeKind::try_new("kind1").unwrap(),
        ).unwrap(),
        domain::Room::try_new(
            domain::Id::new("room1"),
            domain::PlayerCount::try_new(3).unwrap(),
            domain::WolfCount::try_new(1).unwrap(),
            domain::Id::new("host1"),
            vec![domain::Id::new("new_player1"), domain::Id::new("player1")],
            domain::GameMinutes::try_new(3).unwrap(),
            domain::ThemeKind::try_new("kind1").unwrap(),
        ).unwrap()
        =>
        Ok(dto::Room::new(
            "room1".into(),
            3,
            1,
            "host1".into(),
            vec!["new_player1".into(), "player1".into()],
            3,
            "kind1".into(),
        ))
    ; "success")]
    #[async_std::test]
    async fn join_room_ok_works(
        join_command: command::RoomJoin,
        expect_room_id: domain::Id<domain::Room>,
        return_room: domain::Room,
        expect_room_store: domain::Room,
    ) -> Result<dto::Room> {
        let mock_room_service = domain::MockRoomService::new();
        let mut mock_room_repository = domain::MockRoomRepository::new();
        let mock_room_factory = domain::MockRoomFactory::new();

        mock_room_repository
            .expect_find()
            .with(predicate::eq(expect_room_id))
            .returning(move |_| Ok(return_room.clone()));
        mock_room_repository
            .expect_store()
            .with(predicate::eq(expect_room_store))
            .returning(|_| Ok(()));

        let room_usecase = RoomImpl::<MockParameters>::new(
            mock_room_service,
            mock_room_repository,
            mock_room_factory,
        );
        room_usecase.join(join_command).await
    }

    #[test_case(
        command::RoomLeave::new("room1".into(), "leave_player1".into()),
        domain::Id::new("room1"),
        domain::Room::try_new(
            domain::Id::new("room1"),
            domain::PlayerCount::try_new(3).unwrap(),
            domain::WolfCount::try_new(1).unwrap(),
            domain::Id::new("host1"),
            vec![domain::Id::new("leave_player1"),domain::Id::new("player1")],
            domain::GameMinutes::try_new(3).unwrap(),
            domain::ThemeKind::try_new("kind1").unwrap(),
        ).unwrap(),
        domain::Room::try_new(
            domain::Id::new("room1"),
            domain::PlayerCount::try_new(3).unwrap(),
            domain::WolfCount::try_new(1).unwrap(),
            domain::Id::new("host1"),
            vec![domain::Id::new("player1")],
            domain::GameMinutes::try_new(3).unwrap(),
            domain::ThemeKind::try_new("kind1").unwrap(),
        ).unwrap()
        =>
        Ok(dto::Room::new(
            "room1".into(),
            3,
            1,
            "host1".into(),
            vec!["player1".into()],
            3,
            "kind1".into(),
        ))
    ; "success")]
    #[async_std::test]
    async fn leave_room_ok_works(
        leave_command: command::RoomLeave,
        expect_room_id: domain::Id<domain::Room>,
        return_room: domain::Room,
        expect_room_store: domain::Room,
    ) -> Result<dto::Room> {
        let mock_room_service = domain::MockRoomService::new();
        let mut mock_room_repository = domain::MockRoomRepository::new();
        let mock_room_factory = domain::MockRoomFactory::new();

        mock_room_repository
            .expect_find()
            .with(predicate::eq(expect_room_id))
            .returning(move |_| Ok(return_room.clone()));
        mock_room_repository
            .expect_store()
            .with(predicate::eq(expect_room_store))
            .returning(|_| Ok(()));

        let room_usecase = RoomImpl::<MockParameters>::new(
            mock_room_service,
            mock_room_repository,
            mock_room_factory,
        );
        room_usecase.leave(leave_command).await
    }

    #[test_case(
        command::StartGame::new("room1".into()),
        domain::Id::new("room1"),
        domain::Room::try_new(
            domain::Id::new("room1"),
            domain::PlayerCount::try_new(3).unwrap(),
            domain::WolfCount::try_new(1).unwrap(),
            domain::Id::new("host1"),
            vec![domain::Id::new("leave_player1"),domain::Id::new("player1")],
            domain::GameMinutes::try_new(3).unwrap(),
            domain::ThemeKind::try_new("kind1").unwrap(),
        ).unwrap(),
        domain::Game::try_new(
            domain::Id::new("game1"),
            domain::Id::new("room1"),
            domain::Id::new("theme1"),
            datetime(2021,3,4,2,1,3),
            domain::WolfGroup::new(vec![domain::Id::new("player1")], domain::Word::try_new("word1").unwrap()),
            domain::CitizenGroup::new(vec![domain::Id::new("player2")], domain::Word::try_new("word2").unwrap()),
            domain::VoteBox::new(vec![]),
            domain::GameStatus::Talking,
        ).unwrap()
        =>
        Ok(dto::Game::new(
            "game1".into(),
            "room1".into(),
            "theme1".into(),
            datetime(2021,3,4,2,1,3),
            dto::Group::new(vec!["player1".into()],"word1".into()),
            dto::Group::new(vec!["player2".into()],"word2".into()),
            dto::VoteBox::new(vec![]),
            "Talking".into(),
        ))
    ; "success")]
    #[async_std::test]
    async fn start_game_room_ok_works(
        command: command::StartGame,
        expect_room_id: domain::Id<domain::Room>,
        return_room: domain::Room,
        return_game: domain::Game,
    ) -> Result<dto::Game> {
        let mut mock_room_repository = domain::MockRoomRepository::new();
        let mut mock_room_service = domain::MockRoomService::new();
        let mock_room_factory = domain::MockRoomFactory::new();

        let expected_room = return_room.clone();
        mock_room_repository
            .expect_find()
            .with(predicate::eq(expect_room_id))
            .returning(move |_| Ok(return_room.clone()));
        mock_room_service
            .expect_start_game()
            .with(predicate::eq(expected_room))
            .returning(move |_| Ok(return_game.clone()));

        let room_usecase = RoomImpl::<MockParameters>::new(
            mock_room_service,
            mock_room_repository,
            mock_room_factory,
        );
        room_usecase.start_game(command).await
    }
    fn datetime(year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> DateTime<Tz> {
        chrono_tz::Japan
            .ymd(year, month, day)
            .and_hms(hour, min, sec)
    }
}
