use anyhow::anyhow;

use sqlx::sqlite::SqlitePoolOptions;

use std::env;
use std::ffi::OsString;
use std::fs;

#[derive(Debug)]
struct ProjectsDir(pub OsString);

impl ProjectsDir {
  fn from_env() -> anyhow::Result<Self> {
    Ok(Self(OsString::from(env::var("PROJECTS_DIR")?)))
  }
}

#[derive(sqlx::FromRow)]
struct Entry {
  id: u32,
  title: String,
  body: String,
  published: bool,
}

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello {}!", name)
}

async fn create_project(
  name: OsString,
  dir: &ProjectsDir,
) -> anyhow::Result<()> {
  let mut file_name = dir.0.clone();
  file_name.push(name);
  file_name.push(".db");

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

  let mut db = OsString::from("sqlite:");
  db.push(&file_name);

  let pool = SqlitePoolOptions::new()
    .max_connections(1)
    .connect(db.to_str().ok_or(anyhow!("path to db is faulty"))?)
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
  name: OsString,
  dir: &ProjectsDir,
) -> anyhow::Result<()> {
  let mut file_name = dir.0.clone();
  file_name.push(name);
  file_name.push(".db");

  Ok(fs::remove_file(file_name)?)
}

fn list_projects(dir: &ProjectsDir) -> anyhow::Result<Vec<OsString>> {
  let mut projects = Vec::new();

  for entry in fs::read_dir(&dir.0)? {
    let file_name = entry?.file_name();

    let file_ext = file_name
      .to_str()
      .ok_or(anyhow!("error reading file"))?
      .split(".")
      .last()
      .ok_or(anyhow!("error reading file"))?;

    if file_ext == "db" {
      projects.push(file_name);
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

pub fn app() -> anyhow::Result<tauri::App<tauri::Wry>> {
  Ok(
    tauri::Builder::default()
      .manage(ProjectsDir::from_env()?)
      .invoke_handler(tauri::generate_handler![greet])
      .build(tauri::generate_context!())?,
  )
}

#[cfg(test)]
mod tests {
  use std::ffi::OsString;
  use std::fs;

  use super::{
    create_project, delete_project, greet, list_projects, ProjectsDir,
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

    assert_eq!(list_projects(&pd).unwrap(), Vec::<OsString>::new(),);

    // add a project

    create_project("test 1".into(), &pd).await.unwrap();

    assert_eq!(
      list_projects(&pd).unwrap(),
      vec![OsString::from("test 1.db")],
    );

    // add random file, make sure it is excluded from test

    fs::File::create(format!(
      "{}/not a db.txt",
      pd.0.to_str().unwrap()
    ))
    .unwrap();

    assert_eq!(
      list_projects(&pd).unwrap(),
      vec![OsString::from("test 1.db")],
    );

    // remove random file again

    fs::remove_file(format!(
      "{}/not a db.txt",
      &pd.0.to_str().unwrap()
    ))
    .unwrap();

    // delete project created above

    delete_project("test 1".into(), &pd).unwrap();

    assert_eq!(list_projects(&pd).unwrap(), Vec::<OsString>::new(),);
  }

  #[test]
  fn test_greet() {
    assert_eq!(greet("World"), "Hello World!".to_owned());
  }
}