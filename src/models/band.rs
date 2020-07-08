use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tokio_pg_mapper_derive::PostgresMapper;
use juniper::{GraphQLObject, GraphQLInputObject};

#[derive(Clone, Serialize, Deserialize, PostgresMapper, GraphQLObject)]
#[pg_mapper(table="bands")]
pub struct Band {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(GraphQLInputObject)]
pub struct CreateBand {
    pub name: String,
    pub owner_id: Uuid,
}