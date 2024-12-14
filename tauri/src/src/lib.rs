use async_graphql::{Response, ServerError};

use tauri::State;

use std::env;

mod api;
mod dal;
mod gql;

use dal::DAL;
use gql::Schema;

#[tauri::command]
async fn graphql(
  query: String,
  schema: State<'_, Schema>,
) -> Result<Response, Vec<ServerError>> {
  schema.execute(query).await.into_result()
}

pub fn schema() -> anyhow::Result<Schema> {
  Ok(Schema::new(DAL::new(env::var("PROJECTS_DIR")?)))
}

pub fn app() -> anyhow::Result<tauri::App<tauri::Wry>> {
  let app = tauri::Builder::default()
    .manage(schema()?)
    .invoke_handler(tauri::generate_handler![graphql])
    .build(tauri::generate_context!())?;

  Ok(app)
}