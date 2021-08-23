use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("{0}")]
    Open(anyhow::Error),
    #[error("{0}")]
    TransactionBegin(anyhow::Error),
    #[error("{0}")]
    TransactionRollback(anyhow::Error),
    #[error("{0}")]
    TransactionCommit(anyhow::Error),
}

#[macro_export]
macro_rules! run_in_transaction {
    ($ex:expr,$tx:ident,$s:block) => {{
        let mut $tx = $ex.begin().await?;
        let r = $s;
        if r.is_ok() {
            $tx.commit().await?;
        } else {
            $tx.rollback().await?;
        }
        r
    }};
}

#[async_trait]
pub trait ConnectionFactory {
    type Transaction: Transaction;
    type Connection: Connection<Transaction = Self::Transaction>;

    async fn create(&self) -> Result<Self::Connection, DatabaseError>;
}

#[async_trait]
pub trait Connection {
    type Transaction: Transaction;

    async fn begin(&mut self) -> Result<Self::Transaction, DatabaseError>;
}

#[async_trait]
pub trait Transaction {
    async fn commit(self) -> Result<(), DatabaseError>;
    async fn rollback(self) -> Result<(), DatabaseError>;
}

/// NOTE(ryutah): Datastoreの実装時に必要になる
#[allow(dead_code)]
pub enum Executor<'a, C: Connection, Tx: Transaction> {
    Connection(&'a mut C),
    Transaction(&'a mut Tx),
}

impl PartialEq for DatabaseError {
    fn eq(&self, t: &Self) -> bool {
        matches!(
            (self, t),
            (DatabaseError::Open(_), DatabaseError::Open(_))
                | (
                    DatabaseError::TransactionBegin(_),
                    DatabaseError::TransactionBegin(_),
                )
                | (
                    DatabaseError::TransactionRollback(_),
                    DatabaseError::TransactionRollback(_)
                )
                | (
                    DatabaseError::TransactionCommit(_),
                    DatabaseError::TransactionCommit(_)
                )
        )
    }
}
