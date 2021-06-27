use serde_derive::{Serialize};
use crate::schema::posts;

#[derive(Serialize, Debug, Clone, Queryable)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub author: String,
    pub body: Option<String>,
    pub published: bool,
}

#[derive(Debug, Clone, Insertable)]
#[table_name = "posts"]
pub struct CreatePost {
    pub title: String,
    pub author: String,
    pub body: Option<String>,
    pub published: bool,
}
/*
#[#[derive(Serialize, Deserialize, Debug, Copy, Clone, AsExpression, FromSqlRow)]]
#[sql_type = "Text"]
pub enum PostStatus {
    draft,
    published,
    unpublished,
}
impl ToSql<Text, Pg> for {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            PostStatus::draft => out.write_all(b"DRAFT")?,
            PostStatus::published => out.write_all(b"PUBLISHED")?,
            PostStatus::unpublished => out.write_all(b"UNPUBLISHED")?,
        }
        Ok(IsNull::No)
    }
}
impl FromSql<Text, Pg> for PostStatus {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"DRAFT" => Ok(PostStatus::draft),
            b"PUBLISHED" => Ok(PostStatus::published),
            b"UNPUBLISHED" => Ok(PostStatus::unpublished),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}*/