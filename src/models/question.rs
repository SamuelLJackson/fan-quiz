use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tokio_pg_mapper_derive::PostgresMapper;
use juniper::GraphqlInputObject;

#[derive(Clone, Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table="questions")]
pub struct Question {
    pub id: Uuid,
    pub content: String,
    
    pub correct_answer_id: Uuid,
    pub band_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(GraphqlInputObject)]
pub struct CreateQuestion {
    pub content: String,
    pub correct_answer_id: Uuid,
    pub band_id: Uuid,
}