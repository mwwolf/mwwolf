use super::*;

pub struct ThemeRepository<CF: database::ConnectionFactory> {
    connection_factory: CF,
}

#[async_trait]
impl<CF: database::ConnectionFactory> domain::ThemeRepository for ThemeRepository<CF> {
    async fn find_by_kind(
        &self,
        _: &domain::ThemeKind,
    ) -> domain::RepositoryResult<Vec<domain::Theme>> {
        // query 生成
        // datastore Serch実行
        // オブジェクト変換
        todo!()
    }
}
