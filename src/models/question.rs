use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tokio_pg_mapper_derive::PostgresMapper;
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(Clone, Serialize, Deserialize, PostgresMapper, GraphQLObject)]
#[pg_mapper(table="questions")]
pub struct Question {
    pub id: Uuid,
    pub content: String,
    
    pub correct_answer_id: Uuid,
    pub band_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(GraphQLInputObject)]
pub struct CreateQuestion {
    pub content: String,
    pub correct_answer_id: Uuid,
    pub band_id: Uuid,
}