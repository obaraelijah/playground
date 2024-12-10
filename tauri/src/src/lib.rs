use anyhow::anyhow;

use async_graphql::{
  EmptySubscription, InputObject, Object, Request, Response, Schema,
  ServerError, SimpleObject,
};

use sqlx::sqlite::SqlitePoolOptions;

use tauri::State;

use std::env;
use std::fs;

struct Query;

#[Object]
impl Query {
  async fn projects(&self) -> Vec<String> {
    todo!()
  }

  async fn entries(&self, project: String) -> Vec<Entry> {
    todo!()
  }

  async fn entry(&self, project: String, id: u32) -> Option<Entry> {
    todo!()
  }
}

struct Mutation;

#[Object]
impl Mutation {
  async fn create_project(&self, project: String) -> bool {
    todo!()
  }

  async fn delete_project(&self, project: String) -> bool {
    todo!()
  }

  async fn create_entry(
    &self,
    project: String,
    entry: Entry,
  ) -> bool {
    todo!()
  }

  async fn delete_entry(&self, project: String, id: u32) -> bool {
    todo!()
  }
}

#[derive(Debug)]
struct ProjectsDir(pub String);

impl ProjectsDir {
  fn from_env() -> anyhow::Result<Self> {
    Ok(Self(env::var("PROJECTS_DIR")?))
  }
}

// data access layer:
//
// * Projects
// * Project (db connection)
// * ProjectCache
//
// GQL api:
//
// * Query
// * Mutation
//
// GQL api needs access to data access layer
// (Projects, Project and ProjectCache)

#[derive(sqlx::FromRow, SimpleObject, InputObject)]
struct Entry {
  id: u32,
  title: String,
  body: String,
  published: bool,
}

async fn create_project(
  name: String,
  dir: &ProjectsDir,
) -> anyhow::Result<()> {
  let file_name = format!("{}/{}.db", dir.0, name);

  // File must be created first, so that the pool can connect.
  //
  // Also, this call fails if the project already exists, making sure
  // we don't accidentally override an existing project.
  //
  {
    fs::File::options()
      .read(true)
      .write(true)
      .create_new(true)
      .open(&file_name)?;
  }

  let db = format!("sqlite:{}", file_name);

  let pool = SqlitePoolOptions::new()
    .max_connections(1)
    .connect(&db)
    .await?;

  sqlx::query(
    "
      CREATE TABLE entries (
        id INTEGER PRIMARY KEY NOT NULL,
        title TEXT NOT NULL,
        body TEXT NOT NULL,
        published NOT NULL
      );
    ",
  )
  .execute(&pool)
  .await?;

  Ok(())
}

fn delete_project(
  name: String,
  dir: &ProjectsDir,
) -> anyhow::Result<()> {
  let file_name = format!("{}/{}.db", dir.0, name);
  Ok(fs::remove_file(file_name)?)
}

fn list_projects(dir: &ProjectsDir) -> anyhow::Result<Vec<String>> {
  let mut projects = Vec::new();

  for entry in fs::read_dir(&dir.0)? {
    let file_name = entry?.file_name();

    let file_ext = file_name
      .to_str()
      .ok_or(anyhow!("error reading file name"))?
      .split(".")
      .last()
      .ok_or(anyhow!("error deconstructing file name"))?;

    if file_ext == "db" {
      projects.push(file_name.into_string().map_err(|_| {
        anyhow!("couldn't parse file name into valid UTF-8")
      })?);
    }
  }

  Ok(projects)
}

async fn insert_entry() -> anyhow::Result<()> {
  // here access cached connection pool to project
  todo!()
}

async fn delete_entry() -> anyhow::Result<()> {
  // here access cached connection pool to project
  todo!()
}

async fn list_entries() -> anyhow::Result<()> {
  // here access cached connection pool to project
  todo!()
}

#[tauri::command]
async fn graphql(
  query: Request,
  schema: State<'_, Schema<Query, Mutation, EmptySubscription>>,
) -> Result<Response, Vec<ServerError>> {
  schema.execute(query).await.into_result()
}

pub fn app() -> anyhow::Result<tauri::App<tauri::Wry>> {
  Ok(
    tauri::Builder::default()
      .manage(ProjectsDir::from_env()?)
      .manage(
        Schema::build(Query, Mutation, EmptySubscription).finish(),
      )
      .invoke_handler(tauri::generate_handler![graphql])
      .build(tauri::generate_context!())?,
  )
}

#[cfg(test)]
mod tests {
  use std::fs;

  use super::{
    create_project, delete_project, list_projects, ProjectsDir,
  };

  #[tokio::test]
  async fn test_duplicated_project() {
    dotenv::dotenv().unwrap();

    let pd = ProjectsDir::from_env().unwrap();

    create_project("test dublicated".into(), &pd).await.unwrap();

    // second call should fail, because project already exists

    assert!(create_project("test dublicated".into(), &pd)
      .await
      .is_err());

    // clean up projects after test

    delete_project("test dublicated".into(), &pd).unwrap();
  }

  #[tokio::test]
  async fn test_list_projects() {
    dotenv::dotenv().unwrap();

    let pd = ProjectsDir::from_env().unwrap();

    assert_eq!(list_projects(&pd).unwrap(), Vec::<String>::new());

    // add a project

    create_project("test 1".into(), &pd).await.unwrap();

    assert_eq!(
      list_projects(&pd).unwrap(),
      vec!["test 1.db".to_owned()],
    );

    // add random file, make sure it is excluded from test

    fs::File::create(format!("{}/not a db.txt", pd.0)).unwrap();

    assert_eq!(
      list_projects(&pd).unwrap(),
      vec!["test 1.db".to_owned()],
    );

    // remove random file again

    fs::remove_file(format!("{}/not a db.txt", pd.0)).unwrap();

    // delete project created above

    delete_project("test 1".into(), &pd).unwrap();

    assert_eq!(list_projects(&pd).unwrap(), Vec::<String>::new());
  }
}