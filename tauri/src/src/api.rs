use sqlx::FromRow;

use async_graphql::{InputObject, SimpleObject};

#[derive(FromRow, SimpleObject, InputObject, Default)]
pub struct Entry {
    pub id: u32,
    pub title: String,
    pub body: String,
    pub published: bool,
}