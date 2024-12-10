use sqlx::FromRow;

use async_graphql::{InputObject, SimpleObject};

#[derive(FromRow, SimpleObject, InputObject)]
pub struct Entry {
  id: u32,
  title: String,
  body: String,
  published: bool,
}