use sqlx::FromRow;

use async_graphql::{InputObject, SimpleObject};

#[derive(FromRow, SimpleObject, InputObject, Debug, PartialEq)]
pub struct Entry {
    pub id: Option<u32>,
    pub title: String,
    pub body: String,
    pub published: bool,
}
