use redis::{Client};
use sqlx::{Pool, Postgres};

use crate::errors::AppError;

pub type Database = Pool<Postgres>;
pub type Redis = Client;

pub type Token = String;

pub type DatabaseResult<T> = Result<T, sqlx::Error>;
pub type AppResult<T> = Result<T, AppError>;
