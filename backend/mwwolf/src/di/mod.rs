pub mod local;

mod cloudrun;
mod repository;
// mod usecase_injector;

// use async_std::sync::Arc;
#[cfg(feature = "local")]
pub use local::*;

#[cfg(feature = "cloudrun")]
pub use cloudrun::*;

// pub use usecase_injector::*;

use async_graphql::ObjectType;
use async_graphql::Schema;
use async_graphql::SubscriptionType;

pub struct Injector;

impl Injector {}

pub fn create_schema<Query, Mutation, Subscription>(
    query: Query,
    mutaion: Mutation,
    subscription: Subscription,
) -> Schema<Query, Mutation, Subscription>
where
    Query: ObjectType + 'static,
    Mutation: ObjectType + 'static,
    Subscription: SubscriptionType + 'static,
{
    let cf = create_connection_factory("xxxxxxxxxxxxxxxxxxxxxx".into());
    Schema::build(query, mutaion, subscription)
        .data(cf)
        .finish()
}
