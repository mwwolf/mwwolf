use libmww::id::Id;

use crate::{DomainError, DomainErrorKind, DomainResult};

#[derive(EnumString, IntoStaticStr)]
pub enum PlayerKind {
    #[strum(serialize = "host")]
    Host,
    #[strum(serialize = "guest")]
    Guest,
}

#[derive(Debug, PartialEq)]
pub struct PlayerName(String);

impl From<PlayerName> for String {
    fn from(n: PlayerName) -> Self {
        n.0
    }
}

impl PlayerName {
    pub fn try_new(name: impl Into<String>) -> DomainResult<PlayerName> {
        let name = name.into();
        if name.is_empty() {
            Err(DomainError::new(
                DomainErrorKind::InvalidInput,
                "name should not be blank",
            ))
        } else {
            Ok(Self(name))
        }
    }
}

#[derive(new, Getters)]
pub struct Player {
    id: Id<Player>,
    kind: PlayerKind,
    name: PlayerName,
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_case::test_case;

    #[test_case("name" => Ok(PlayerName("name".into())))]
    #[test_case("" => Err(DomainError::new(DomainErrorKind::InvalidInput, "name should not be blank")))]
    fn player_name_try_new_test(name: &str) -> DomainResult<PlayerName> {
        PlayerName::try_new(name)
    }

    #[test_case(PlayerName("name".into()) => "name".to_string())]
    fn player_name_into_string(player_name: PlayerName) -> String {
        player_name.into()
    }
}
