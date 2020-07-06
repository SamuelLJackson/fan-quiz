use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tokio_pg_mapper_derive::PostgresMapper;
use juniper::{GraphQLObject, GraphQLInputObject};

#[derive(Clone, Serialize, Deserialize, PostgresMapper, GraphQLObject)]
#[pg_mapper(table="answers")]
pub struct Answer {
    pub id: Uuid,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(GraphQLInputObject)]
pub struct CreateAnswer {
    pub content: String,
}