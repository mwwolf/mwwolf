use async_graphql::{EmptySubscription, Schema};

// #[macro_use]
extern crate derive_new;
#[macro_use]
extern crate async_graphql;

#[derive(MergedObject, Default)]
pub struct Query();

#[derive(MergedObject, Default)]
pub struct Mutation();

pub type KzSchema = Schema<Query, Mutation, EmptySubscription>;
