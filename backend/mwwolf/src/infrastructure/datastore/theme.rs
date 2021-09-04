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
        let conn = self
            .connection_factory
            .create()
            .await
            .map_err(to_repository_error)?;
        // query 生成 -> DONE
        // Connectionにquery実行する実装がなかったのでdatastore::Connectionにquery実行するメソッドを実装する
        // datastore Serch実行
        // オブジェクト変換
        todo!()
    }
}
