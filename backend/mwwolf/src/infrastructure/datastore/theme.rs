use std::collections::HashMap;

use super::*;
use database::ConnectionFactory as _;

#[derive(new)]
pub struct ThemeRepository {
    connection_factory: ConnectionFactory,
    namespace: String,
}

impl ThemeRepository {
    const DATA_STORE_KIND: &'static str = "theme";
    const KIND_FIELD_NAME: &'static str = "kind";
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
                Self::KIND_FIELD_NAME.into(),
                proto_api::Value::Strings(kind.raw_kind().into()),
            ));
        let mut conn = self
            .connection_factory
            .create()
            .await
            .map_err(to_repository_error)?;
        let entities = conn.query(query).await.map_err(|e| {
            domain::RepositoryError::new_with_source(
                domain::RepositoryErrorKind::Fail,
                format!("failed to search theme by kind: {}", kind.raw_kind()),
                e.into(),
            )
        })?;

        for e in entities.into_iter() {
            let props = HashMap::<String, proto_api::Value>::from_value(e.into_properties())
                .map_err(|e| {
                    domain::RepositoryError::new_with_source(
                        domain::RepositoryErrorKind::Fail,
                        format!("failed to convert value to hashmap. entity: {:?}", e),
                        e.into(),
                    )
                })?;
        }

        // query 生成 -> DONE
        // Connectionにquery実行する実装がなかったのでdatastore::Connectionにquery実行するメソッドを実装する
        // datastore Serch実行
        // オブジェクト変換
        todo!()
    }
}

struct ThemeFields;
impl ThemeFields {
    const KIND: &'static str = "kind";
    const FIRST: &'static str = "first";
    const SECOND: &'static str = "second";
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
