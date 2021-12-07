mod context;
mod errors;
mod query;
mod services;
mod subscription;
mod types;

pub use context::GraphqlContext;
pub use errors::Error;

use crate::query::Query;
use crate::subscription::Subscription;

pub use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Data,
};

use async_graphql::{EmptyMutation, Schema, SchemaBuilder};

pub fn new_schema() -> SchemaBuilder<Query, EmptyMutation, Subscription> {
    Schema::build(
        Query::default(),
        EmptyMutation::default(),
        Subscription::default(),
    )
}
