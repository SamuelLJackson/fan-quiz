use deadpool_postgres::{Pool, Client};
use std::{collections::HashMap, sync::Arc};
use slog_scope::{error, info};
use crate::models::question::{Question, CreateQuestion};
use tokio_pg_mapper::FromTokioPostgresRow;
use crate::errors::{AppError, AppErrorType};
use tokio_postgres::error::{Error, SqlState};
use uuid::Uuid;
use async_trait::async_trait;
use dataloader::{BatchFn, cached::Loader};

pub struct QuestionRepository {
    pool: Arc<Pool>,
}

pub struct QuestionBatcher {
    pool: Arc<Pool>,
}

pub type QuestionLoader = Loader<Uuid, Vec<Question>, AppError, QuestionBatcher>;

pub fn get_question_loader(pool: Arc<Pool>) -> QuestionLoader {
    Loader::new(QuestionBatcher { pool })
        .with_yield_count(100)
}

impl QuestionRepository {

    pub fn new(pool: Arc<Pool>) -> QuestionRepository {
        QuestionRepository { pool }
    }

    pub async fn get(&self, id: Uuid) -> Result<Question, AppError> {
        let client: Client = self.pool
            .get()
            .await
            .map_err(|err| {
                error!("Error getting client {}", err; "query" => "get");
                err
            })?;

        let statement = client.prepare("select * from questions where id = $1").await?;

        client
            .query(&statement, &[&id])
            .await
            .map_err(|err| {
                error!("Error getting users. {}", err; "query" => "get");
                err
            })?
            .iter()
            .map(|row| Question::from_row_ref(row))
            .collect::<Result<Vec<Question>, _>>()?
            .pop()
            .ok_or(AppError {
                cause: None,
                message: None,
                error_type: AppErrorType::NotFoundError
            })
    }

    pub async fn all(&self) -> Result<Vec<Question>, AppError> {
        let client: Client = self.pool
            .get()
            .await
            .map_err(|err| {
                error!("Error getting client {}", err; "query" => "all");
                err
            })?;

        let statement = client.prepare("select * from questions").await?;

        let users = client
            .query(&statement, &[])
            .await
            .map_err(|err| {
                error!("Error getting users. {}", err; "query" => "all");
                err
            })?
            .iter()
            .map(|row| Question::from_row_ref(row))
            .collect::<Result<Vec<Question>, _>>()
            .map_err(|err| {
                error!("Error getting parsing users. {}", err; "query" => "all");
                err
            })?;

        Ok(users)
    }

    #[allow(dead_code)]
    pub async fn get_for_user(&self, user_id: Uuid) -> Result<Vec<Question>, AppError> {
        let client: Client = self.pool
            .get()
            .await
            .map_err(|err| {
                error!("Error getting client {}", err; "query" => "get_for_user");
                err
            })?;

        let statement = client.prepare("select * from questions where author_id = $1").await?;

        let users = client
            .query(&statement, &[&user_id])
            .await
            .map_err(|err| {
                error!("Error getting users. {}", err; "query" => "get_for_user");
                err
            })?
            .iter()
            .map(|row| Question::from_row_ref(row))
            .collect::<Result<Vec<Question>, _>>()
            .map_err(|err| {
                error!("Error getting parsing users. {}", err; "query" => "get_for_user");
                err
            })?;

        Ok(users)
    }

    pub async fn create(&self, input: CreateQuestion) -> Result<Question, AppError> {
        let client: Client = self.pool
            .get()
            .await
            .map_err(|err| {
                error!("Error getting client {}", err; "query" => "create");
                err
            })?;

        let statement = client
            .prepare("insert into questions (author_id, slug, title, description, body) values ($1, $2, $3, $4, $5) returning *")
            .await?;

        let question = client.query(&statement, &[
                &input.content,
                &input.band_id,
                &input.correct_answer_id,
            ])
            .await
            .map_err(|err: Error| {
                match err.code() {
                    Some(code) => match code {
                        c if c == &SqlState::UNIQUE_VIOLATION => AppError {
                            cause: Some(err.to_string()),
                            message: Some(format!("question {} already exists.", &input.content)),
                            error_type: AppErrorType::InvalidField
                        },
                        c if c == &SqlState::FOREIGN_KEY_VIOLATION=> AppError {
                            cause: Some(err.to_string()),
                            message: Some(format!("band with id {} doesn't exists.", &input.band_id)),
                            error_type: AppErrorType::InvalidField
                        },
                        _ => AppError::from(err)
                    }
                    _ => AppError::from(err)
                }
            })?
            .iter()
            .map(|row| Question::from_row_ref(row))
            .collect::<Result<Vec<Question>, _>>()?
            .pop()
            .ok_or(AppError {
                message: Some("Error creating Question.".to_string()),
                cause: Some("Unknown error.".to_string()),
                error_type: AppErrorType::DbError,
            })?;

        Ok(question)
    }
}

impl QuestionBatcher {
    pub async fn get_questions_by_band_ids(&self, hashmap: &mut HashMap<Uuid, Vec<Question>>, ids: Vec<Uuid>) -> Result<(), AppError> {
        let client: Client = self.pool
            .get()
            .await
            .map_err(|err| {
                error!("Error getting client {}", err; "query" => "get_questions_by_band_ids");
                err
            })?;

        let statement = client.prepare("select * from questions where author_id = ANY($1)").await?; 

        client
            .query(&statement, &[&ids])
            .await
            .map_err(|err| {
                error!("Error getting questions. {}", err; "query" => "get_questions_by_band_ids");
                err
            })?
            .iter()
            .map(|row| Question::from_row_ref(row))
            .collect::<Result<Vec<Question>, _>>()
            .map_err(|err| {
                error!("Error getting parsing questions. {}", err; "query" => "get_questions_by_band_ids");
                err
            })?
            .iter()
            .fold(
                hashmap,
                |map: &mut HashMap<Uuid, Vec<Question>>, question: &Question| {
                    let vec = map
                        .entry(question.band_id)
                        .or_insert_with(|| Vec::<Question>::new());
                    vec.push(question.clone());
                    map
                }
            );

        Ok(())

    }
}

#[async_trait]
impl BatchFn<Uuid, Vec<Question>> for QuestionBatcher {
    type Error = AppError;

    async fn load(&self, keys: &[Uuid]) -> HashMap<Uuid, Result<Vec<Question>, AppError>> {

        info!("Loading batch {:?}", keys);

        let mut questions_map = HashMap::new();

        let result: Result<(), AppError> = self.get_questions_by_band_ids(&mut questions_map, keys.into()).await;

        keys
            .iter()
            .map(move |id| {
                let entry = 
                    questions_map.entry(*id)
                        .or_insert_with(|| vec![])
                        .clone();

                    (id.clone(), result.clone().map(|_| entry))
                })
                .collect::<HashMap<_, _>>()
    }
}