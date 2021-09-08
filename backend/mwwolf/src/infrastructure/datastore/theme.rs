use super::*;
use database::ConnectionFactory as _;

#[derive(new)]
pub struct ThemeRepository {
    connection_factory: ConnectionFactory,
    namespace: String,
}

impl ThemeRepository {
    const DATA_STORE_KIND: &'static str = "theme";
}

struct ThemeFields;
impl ThemeFields {
    const KIND: &'static str = "kind";
    const FIRST: &'static str = "first";
    const SECOND: &'static str = "second";
}

#[async_trait]
impl domain::ThemeRepository for ThemeRepository {
    async fn find_by_kind(
        &self,
        kind: &domain::ThemeKind,
    ) -> domain::RepositoryResult<Vec<domain::Theme>> {
        let query = proto_api::Query::new(Self::DATA_STORE_KIND)
            .namespace(&self.namespace)
            .filter(proto_api::Filter::Equal(
                ThemeFields::KIND.into(),
                proto_api::Value::Strings(kind.raw_kind().into()),
            ));
        let mut conn = self
            .connection_factory
            .create()
            .await
            .map_err(to_repository_error)?;
        conn.query(query).await.map_err(|e| {
            domain::RepositoryError::new_with_source(
                domain::RepositoryErrorKind::Fail,
                format!("failed to search theme by kind: {}", kind.raw_kind()),
                e.into(),
            )
        })
    }
}

impl proto_api::FromEntity for domain::Theme {
    fn from_entity(e: proto_api::Entity) -> std::result::Result<Self, proto_api::ConvertError> {
        let id: domain::Id<domain::Theme> = id::key_to_id(e.key().clone());
        let props = e.into_properties();
        if let proto_api::Value::Entity(vmap) = props {
            let (vmap, kind) = entity::remove_value::<String>(vmap, ThemeFields::KIND)?;
            let (vmap, first) = entity::remove_value::<String>(vmap, ThemeFields::FIRST)?;
            let (_, second) = entity::remove_value::<String>(vmap, ThemeFields::SECOND)?;
            Ok(domain::Theme::new(
                id,
                domain::ThemeKind::try_new(kind).unwrap(),
                domain::Word::try_new(first).unwrap(),
                domain::Word::try_new(second).unwrap(),
            ))
        } else {
            Err(proto_api::ConvertError::UnexpectedPropertyType {
                expected: "entity".into(),
                got: props.type_name().into(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, iter::FromIterator};

    use super::*;
    use test_case::test_case;

    fn new_theme(id: &str, kind: &str, first_word: &str, second_word: &str) -> domain::Theme {
        domain::Theme::new(
            domain::Id::new(id),
            domain::ThemeKind::try_new(kind).unwrap(),
            domain::Word::try_new(first_word).unwrap(),
            domain::Word::try_new(second_word).unwrap(),
        )
    }

    fn new_theme_entity(
        id: i64,
        theme_kind: &str,
        first_word: &str,
        second_word: &str,
    ) -> proto_api::Entity {
        let properties = HashMap::<_, _>::from_iter(
            [
                (ThemeFields::KIND, theme_kind),
                (ThemeFields::FIRST, first_word),
                (ThemeFields::SECOND, second_word),
            ]
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect::<Vec<_>>(),
        );
        proto_api::Entity::new(
            Key::new(ThemeRepository::DATA_STORE_KIND).id(id),
            properties,
        )
        .unwrap()
    }

    #[test_case(
        new_theme_entity(1, "kind_test", "first_word", "second_word")
        =>
        Ok(new_theme("1", "kind_test", "first_word", "second_word"))
    )]
    #[test_case(
        new_theme_entity(2, "kind_test_2", "first_word_2", "second_word_2")
        =>
        Ok(new_theme("2", "kind_test_2", "first_word_2", "second_word_2"))
    )]
    fn domain_theme_from_entity_works(
        entity: proto_api::Entity,
    ) -> Result<domain::Theme, proto_api::ConvertError> {
        domain::Theme::from_entity(entity)
    }
}
