use async_graphql::{Request, Response, ServerError};

use tauri::State;

use std::env;

mod api;
mod dal;
mod gql;

use dal::DAL;
use gql::Schema;

// data access layer:
//
// * Projects (ops for projects: list, create, delete)
// * Project (db connection)
// * ProjectCache (Map<String, Project>, shared)
//
// ... single thread-safe DAL struct with cache and all ops.
//     Cache updated via interior mutability (Arc<RwLock<Cache>>)
//
//     ! make sure to use tokio::sync::RwLock !
//

#[tauri::command]
async fn graphql(
    query: Request,
    schema: State<'_, Schema>,
) -> Result<Response, Vec<ServerError>> {
    schema.execute(query).await.into_result()
}

pub fn app() -> anyhow::Result<tauri::App<tauri::Wry>> {
    let dal = DAL::new(env::var("PROJECTS_DIR")?);
    let schema = Schema::new(dal);

    let app = tauri::Builder::default()
        .manage(schema)
        .invoke_handler(tauri::generate_handler![graphql])
        .build(tauri::generate_context!())?;

    Ok(app)
}
