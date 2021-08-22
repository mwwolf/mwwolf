use async_graphql::{EmptySubscription, Schema};

#[derive(MergedObject, Default)]
pub struct Query();

#[derive(MergedObject, Default)]
pub struct Mutation();

pub type KzSchema = Schema<Query, Mutation, EmptySubscription>;
