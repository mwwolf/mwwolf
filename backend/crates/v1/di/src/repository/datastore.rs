use async_std::sync::Arc;
use infrastructure::datastore;

pub type Connection = datastore::Connection;

pub type Transaction = datastore::Transaction;

pub type ConnectionFactory = datastore::ConnectionFactory;

pub fn create_connection_factory(namespace: String) -> Arc<ConnectionFactory> {
    Arc::new(ConnectionFactory::new(
        std::env::var("GOOGLE_CLOUD_PROJECT").unwrap(),
        namespace,
    ))
}
