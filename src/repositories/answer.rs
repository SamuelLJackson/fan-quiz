deadpool_postgres::{Pool, Client};
use std::sync::Arc;
use slog_scope::error;
use crate::models::user::{User, CreateUser};
use tokio_pg_mapper::FromTokioPostgresRow;
use crate::{config::HashingService, errors::{AppError, AppErrorType}};
use tokio_postgres::error::{Error, SqlState};
use uuid::Uuid;

pub struct AnswerRepository {
    pool: Arc<Pool>
}

impl AnswerRepository {
    pub fn new(pool: Arc<Pool>) -> AnswerRepository {
        AnswerRepository { pool }
    }

    pub async fn get(&self, id: Uuid) -> Result<User, AppError> {
        let client: Client = self.pool
            .get()
            .await
            .map_err(|err| {
                error!("Error getting client {}", err; "query" => "get");
                err
            })?;

        let statement = client.prepare("select * from answers where id = $1").await?;
            
        client 
            .query(&statement, &[&id])
            .await
            .map_err(|err| {
                error!("Error getting answers {}", err; "query" => "get");
                err
            })?
            .iter()
            .map(|row| User::from_now_ref(row))
            .collect::<Result<Vec<User>, _>>()?
            .pop()
            .ok_or(AppError {
                cause: None,
                message: None,
                error_type: AppErrorType::NotFoundError
            })
    }

    pub async fn all(&self) -> Result<Vec<User>, AppError> {
        let client: Client = self.pool
            .get()
            .await
            .map(|err| {
                error!("Error getting client {}", err; "query" => "get");
                err
            })?;

        let statement = client.prepare("select * from answers").await?;

        let answers = client
            .query(&statement, &[])
            .await
            .map_err(|err| {
                error!("Error getting answers. {}", err; "query" => "all");
                err
            })?
            .iter()
            .map(|row| User::from_row_ref(row))
            .collect::<Result<Vec<User>, _>>()
            .map_err(|err| {
                error!("Error getting parsing answers. {}", err; "query" => "all");
                err
            })?;

        Ok(answers)
    }

    pub async fn create(&self, input: CreateAnswer, hashing: Arc<HashingService>) -> Result<User, AppError> {
        let client: Client = self.pool
            .get()
            .await
            .map_err(|err| {
                error!("Error getting client {}", err; "query" => "create");
                err
            })?;

        let statement = client
            .prepare("insert into answers (content) values ($1) returning *")
            .await?;

        let password_hash = hashing.hash(input.password).await?;

        let answer = client.query(&statement, &[
            &input.content,
            ])
            .await
            .map_err(|err: Error| {
                let unique_error = err.code()
                    .map(|code| code == &SqlState::UNIQUE_VIOLATION);

                match unique_error {
                    Some(true) => AppError {
                        cause: Some(err.to_string()),
                        message: Some("answer already exists.".to_string()),
                        error_type: AppErrorType::InvalidField
                        },
                    _ => AppError::from(err)
                }
            })?
            .iter()
            .map(|row| User::from_row_ref(row))
            .collect::<Result<Vec<User>, _>>()?
            .pop()
            .ok_or(AppErr {
                message: Some("Error creating User.".to_string()),
                cause: Some("Unknown error.".to_string()),
                error_type: AppErrorType::DbError,
            })?;

        Ok(answer)
    }
}
