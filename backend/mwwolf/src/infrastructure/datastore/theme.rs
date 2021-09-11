pub(crate) use super::*;
use database::ConnectionFactory as _;
use std::collections::HashMap;

#[derive(new)]
pub struct ThemeRepository {
    connection_factory: Arc<ConnectionFactory>,
}

// FIXME: remove this for name_of macro!
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
        let query = proto_api::Query::new(entity::kind::<domain::Theme>()).filter(
            proto_api::Filter::Equal(
                ThemeFields::KIND.into(),
                proto_api::Value::Strings(kind.raw_kind().into()),
            ),
        );
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
        let id: domain::Id<domain::Theme> = e.key().clone().into();
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

impl proto_api::IntoEntity for domain::Theme {
    fn into_entity(self) -> Result<proto_api::Entity, proto_api::ConvertError> {
        let key: proto_api::Key = self.id().clone().into();
        proto_api::Entity::new(key, self)
    }
}

impl proto_api::IntoValue for domain::Theme {
    fn into_value(self) -> proto_api::Value {
        let mut properties = HashMap::new();
        properties.insert(
            name_of!(const kind in Self).into(),
            self.kind().raw_kind().into_value(),
        );
        properties.insert(
            name_of!(const first in Self).into(),
            self.first().raw().into_value(),
        );
        properties.insert(
            name_of!(const second in Self).into(),
            self.second().raw().into_value(),
        );
        proto_api::Value::Entity(properties)
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, iter::FromIterator};

    use super::*;
    use domain::ThemeRepository as _;
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
        proto_api::Entity::new(Key::new(entity::kind::<domain::Theme>()).id(id), properties)
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

    #[test_case(domain::ThemeKind::try_new("hoge").unwrap(),
    vec![]=>Ok(vec![]))]
    #[test_case(domain::ThemeKind::try_new("hoge").unwrap(),
    vec![
        domain::Theme::new(domain::Id::new("1234"),domain::ThemeKind::try_new("hoge").unwrap(),
        domain::Word::try_new("first_hoge").unwrap()
        ,domain::Word::try_new("second_hoge").unwrap()),
    ]=>Ok(vec![domain::Theme::new(domain::Id::new("1234"),domain::ThemeKind::try_new("hoge").unwrap(),
        domain::Word::try_new("first_hoge").unwrap()
        ,domain::Word::try_new("second_hoge").unwrap())]);"get_one_theme")]
    #[async_std::test]
    async fn theme_repository_find_by_kind_works(
        kind: domain::ThemeKind,
        givens: Vec<domain::Theme>,
    ) -> domain::RepositoryResult<Vec<domain::Theme>> {
        let datastore = testmww::integration_test::init_test_database()
            .await
            .unwrap();
        theme_repository_find_by_kind_fixtures(givens, datastore.as_ref()).await;
        let theme_repository = ThemeRepository::new(datastore.as_ref().clone());
        theme_repository.find_by_kind(&kind).await
    }

    async fn theme_repository_find_by_kind_fixtures(
        themes: Vec<domain::Theme>,
        datastore: &Arc<ConnectionFactory>,
    ) {
        let mut conn = datastore.create().await.unwrap();
        let entities = themes
            .into_iter()
            .map(|theme| entity::into_entity(theme, &datastore.namespace).unwrap())
            .collect::<Vec<_>>();
        conn.put_all(entities).await.unwrap();
    }
}
