use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::model::{CreatePost, Post};
use crate::errors::{AppError, ErrorType};

type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

pub struct DBAccessManager {
    connection: PooledPg,
}

impl DBAccessManager {
    pub fn new(connection: PooledPg) -> DBAccessManager {
        DBAccessManager { connection }
    }

    pub fn create_post(&self, dto: CreatePost) -> Result<Post, AppError> {
        use super::schema::posts;

        diesel::insert_into(posts::table)
            .values(&dto)
            .get_result(&self.connection)
            .map_err(|err| {
                AppError::from_diesel_err(err,"while creating post")
            })
    }

    pub fn read_posts(&self) -> Result<Vec<Post>, AppError> {
        use super::schema::posts::dsl::*;

        posts
            .load(&self.connection)
            .map_err(|err| {
                AppError::from_diesel_err(err, "while reading posts")
            })
    }

    pub fn delete_post(&self, post_id: i64) -> Result<usize, AppError> {
        use super::schema::posts::dsl::*;

        let deleted = diesel::delete(posts.filter(id.eq(post_id)))
            .execute(&self.connection)
            .map_err(|err| {
                AppError::from_diesel_err(err, "while deleting post")
            })?;

        if deleted == 0 {
            return Err(AppError::new("Post not found", ErrorType::NotFound))
        }
        return Ok(deleted)
    }
}