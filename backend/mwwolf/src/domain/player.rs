use super::*;

#[derive(EnumString, IntoStaticStr)]
pub enum PlayerKind {
    #[strum(serialize = "host")]
    Host,
    #[strum(serialize = "guest")]
    Guest,
}

#[derive(Debug, PartialEq, NamedTupleFrom)]
pub struct PlayerName(String);

impl PlayerName {
    pub fn try_new(name: impl Into<String>) -> DomainResult<Self> {
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
}
