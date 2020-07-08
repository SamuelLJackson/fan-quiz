use deadpool_postgres::{Pool, Client};
use std::sync::Arc;
use slog_scope::error;
use crate::models::band::{Band, CreateBand};
use tokio_pg_mapper::FromTokioPostgresRow;
use crate::errors::{AppError, AppErrorType};
use tokio_postgres::error::{Error, SqlState};
use uuid::Uuid;

pub struct BandRepository {
    pool: Arc<Pool>
}

impl BandRepository {
    pub fn new(pool: Arc<Pool>) -> BandRepository {
        BandRepository { pool }
    }

    pub async fn get(&self, id: Uuid) -> Result<Band, AppError> {
        let client: Client = self.pool
            .get()
            .await
            .map_err(|err| {
                error!("Error getting client {}", err; "query" => "get");
                err
            })?;

        let statement = client.prepare("select * from bands where id = $1").await?;
            
        client 
            .query(&statement, &[&id])
            .await
            .map_err(|err| {
                error!("Error getting bands {}", err; "query" => "get");
                err
            })?
            .iter()
            .map(|row| Band::from_row_ref(row))
            .collect::<Result<Vec<Band>, _>>()?
            .pop()
            .ok_or(AppError {
                cause: None,
                message: None,
                error_type: AppErrorType::NotFoundError
            })
    }

    pub async fn all(&self) -> Result<Vec<Band>, AppError> {
        let client: Client = self.pool
            .get()
            .await
            .map_err(|err| {
                error!("Error getting client {}", err; "query" => "all");
                err
            })?;

        let statement = client.prepare("select * from bands").await?;

        let bands = client
            .query(&statement, &[])
            .await
            .map_err(|err| {
                error!("Error getting bands. {}", err; "query" => "all");
                err
            })?
            .iter()
            .map(|row| Band::from_row_ref(row))
            .collect::<Result<Vec<Band>, _>>()
            .map_err(|err| {
                error!("Error getting parsing bands. {}", err; "query" => "all");
                err
            })?;

        Ok(bands)
    }

    pub async fn create(&self, input: CreateBand) -> Result<Band, AppError> {
        let client: Client = self.pool
            .get()
            .await
            .map_err(|err| {
                error!("Error getting client {}", err; "query" => "create");
                err
            })?;

        let statement = client
            .prepare("insert into bands (content) values ($1) returning *")
            .await?;

        let band = client.query(&statement, &[
            &input.content,
            ])
            .await
            .map_err(|err: Error| {
                let unique_error = err.code()
                    .map(|code| code == &SqlState::UNIQUE_VIOLATION);

                match unique_error {
                    Some(true) => AppError {
                        cause: Some(err.to_string()),
                        message: Some("band already exists.".to_string()),
                        error_type: AppErrorType::InvalidField
                        },
                    _ => AppError::from(err)
                }
            })?
            .iter()
            .map(|row| Band::from_row_ref(row))
            .collect::<Result<Vec<Band>, _>>()?
            .pop()
            .ok_or(AppError {
                message: Some("Error creating Band.".to_string()),
                cause: Some("Unknown error.".to_string()),
                error_type: AppErrorType::DbError,
            })?;

        Ok(band)
    }
}
