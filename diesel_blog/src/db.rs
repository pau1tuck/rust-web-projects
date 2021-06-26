use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::model::{CreatePost, Post};
use crate::errors::{AppError, ErrorType};

type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

pub struct DBAccessManager {
    connection: PooledPg,
}