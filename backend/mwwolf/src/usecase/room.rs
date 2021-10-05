use super::*;

use async_graphql::futures_util::TryFutureExt;
use domain::RoomFactory;
use domain::RoomRepository;

#[async_trait]
trait Room {
    async fn create(&self, command: command::RoomCreate) -> Result<dto::Room>;
    async fn delete(&self, room_id: &str) -> Result<()>;
    async fn join(&self, command: command::RoomJoin) -> Result<dto::Room>;
    async fn leave(&self, palyer_id: &str) -> Result<dto::Room>;
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
    async fn join(&self, _: command::RoomJoin) -> Result<dto::Room> {
        todo!()
    }
    async fn leave(&self, _: &str) -> Result<dto::Room> {
        todo!()
    }
    async fn start_game(&self, _: command::StartGame) -> Result<dto::Game> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
