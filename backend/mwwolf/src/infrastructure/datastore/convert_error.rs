use super::*;
pub fn to_repository_error(db_error: database::DatabaseError) -> domain::RepositoryError {
    domain::RepositoryError::new_with_source(
        domain::RepositoryErrorKind::Fail,
        format!("{}", db_error),
        db_error.into(),
    )
}
