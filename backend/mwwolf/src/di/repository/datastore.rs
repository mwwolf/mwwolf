use crate::infrastructure::datastore;
use async_std::sync::Arc;

pub type Connection = datastore::Connection;

pub type Transaction = datastore::Transaction;

pub type ConnectionFactory = datastore::ConnectionFactory;

pub fn create_connection_factory(namespace: String) -> Arc<ConnectionFactory> {
    Arc::new(ConnectionFactory::new(
        std::env::var("GOOGLE_CLOUD_PROJECT").unwrap(),
        namespace,
    ))
}
