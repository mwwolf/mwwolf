use std::collections::HashMap;

use super::*;
use database::Connection as _;
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

        for e in entities.iter() {
            let props = HashMap::<String, proto_api::Value>::from_value(e.into_properties())
                .map_err(|e| {
                    domain::RepositoryError::new_with_source(
                        domain::RepositoryErrorKind::Fail,
                        format!("failed to convert value to hashmap. entity: {:?}", e),
                        e.into(),
                    )
                })?;
            for (key, val) in props.iter() {}
        }

        // query 生成 -> DONE
        // Connectionにquery実行する実装がなかったのでdatastore::Connectionにquery実行するメソッドを実装する
        // datastore Serch実行
        // オブジェクト変換
        todo!()
    }
}
