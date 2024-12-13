use anyhow::anyhow;

use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

use futures_util::stream::TryStreamExt;

use tokio::sync::RwLock;

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::api::{CreateEntry, Entry};

#[derive(Clone)]
pub struct DAL {
  path: PathBuf,
  cache: Arc<RwLock<HashMap<PathBuf, Project>>>,
}

impl DAL {
  pub fn new<T: Into<PathBuf>>(path: T) -> Self {
    Self {
      path: path.into(),
      cache: Arc::new(RwLock::new(HashMap::new())),
    }
  }

  pub async fn create_project<P: AsRef<Path>>(
    &self,
    project: P,
  ) -> anyhow::Result<Project> {
    let path = self.project_path(&project);

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
        .open(&path)?;
    }

    let p = Project::create(path).await?;

    self.insert_into_cache(project, p.clone()).await;

    Ok(p)
  }

  pub fn delete_project<P: AsRef<Path>>(
    &self,
    project: P,
  ) -> anyhow::Result<()> {
    Ok(fs::remove_file(self.project_path(project))?)
  }

  pub fn projects(&self) -> anyhow::Result<Vec<String>> {
    let mut projects = Vec::new();

    for entry in fs::read_dir(&self.path)? {
      projects.push(entry?.file_name().into_string().map_err(
        |_| anyhow!("couldn't parse file name into valid UTF-8"),
      )?);
    }

    Ok(projects)
  }

  pub async fn project<P: AsRef<Path>>(
    &self,
    project: P,
  ) -> anyhow::Result<Project> {
    match {
      let cache = self.cache.read().await;
      cache.get(project.as_ref()).map(|x| x.clone())
    } {
      Some(p) => Ok(p),
      None => {
        let p = Project::open(self.project_path(&project)).await?;

        self.insert_into_cache(project, p.clone()).await;

        Ok(p)
      }
    }
  }

  async fn insert_into_cache<P: AsRef<Path>>(
    &self,
    key: P,
    val: Project,
  ) {
    let mut cache = self.cache.write().await;
    cache.insert(key.as_ref().to_owned(), val);
  }

  fn project_path<P: AsRef<Path>>(&self, project: P) -> PathBuf {
    let mut path = self.path.clone();
    path.push(project);
    path
  }
}

/// Provides access to the database of a project.
///
/// A [`Project`](Self) is cheaply clonable, because the connection
/// pool itself is just an [`Arc`](Arc) to the underlying connection
/// pool state.
///
#[derive(Clone)]
pub struct Project {
  pool: SqlitePool,
}

impl Project {
  async fn open<P: AsRef<Path>>(file: P) -> anyhow::Result<Self> {
    let pool = SqlitePoolOptions::new()
      .max_connections(1)
      .connect(
        file.as_ref().to_str().ok_or(anyhow!("filename invalid"))?,
      )
      .await?;

    Ok(Self { pool })
  }

  async fn create<P: AsRef<Path>>(file: P) -> anyhow::Result<Self> {
    let instance = Self::open(file).await?;
    instance.create_tables().await?;
    Ok(instance)
  }

  async fn create_tables(&self) -> anyhow::Result<()> {
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
    .execute(&self.pool)
    .await?;

    Ok(())
  }

  pub async fn create_entry(
    &self,
    e: CreateEntry,
  ) -> anyhow::Result<u32> {
    let id = sqlx::query(
      "INSERT INTO entries (title, body, published) VALUES (?, ?, ?);",
    )
    .bind(e.title)
    .bind(e.body)
    .bind(e.published)
    .execute(&self.pool)
    .await?
    .last_insert_rowid();

    Ok(id.try_into()?)
  }

  pub async fn delete_entry(&self, id: u32) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM entries WHERE id = ?")
      .bind(id)
      .execute(&self.pool)
      .await?;

    Ok(())
  }

  pub async fn entries(&self) -> Result<Vec<Entry>, sqlx::Error> {
    sqlx::query_as::<_, Entry>("SELECT * FROM entries;")
      .fetch(&self.pool)
      .try_collect()
      .await
  }

  pub async fn entry(&self, id: u32) -> Result<Entry, sqlx::Error> {
    sqlx::query_as::<_, Entry>("SELECT * FROM entries WHERE id = ?;")
      .bind(id)
      .fetch_one(&self.pool)
      .await
  }
}

#[cfg(test)]
mod tests {
  use std::env;

  use crate::api::{CreateEntry, Entry};

  use super::DAL;

  #[tokio::test]
  async fn test_projects() {
    dotenv::dotenv().unwrap();

    let dir = env::var("PROJECTS_DIR").unwrap();

    let dal = DAL::new(dir);

    assert_eq!(dal.projects().unwrap(), Vec::<String>::new());

    // add a project

    dal.create_project("test 1").await.unwrap();

    assert_eq!(dal.projects().unwrap(), vec!["test 1".to_owned()],);

    // delete project

    dal.delete_project("test 1").unwrap();

    assert_eq!(dal.projects().unwrap(), Vec::<String>::new());
  }

  #[tokio::test]
  async fn test_duplicated_project() {
    dotenv::dotenv().unwrap();

    let dir = env::var("PROJECTS_DIR").unwrap();

    let dal = DAL::new(dir);

    dal.create_project("test dublicated").await.unwrap();

    // second call should fail, because project already exists

    assert!(dal.create_project("test dublicated").await.is_err());

    // clean up projects after test

    dal.delete_project("test dublicated").unwrap();
  }

  #[tokio::test]
  async fn test_entries() {
    dotenv::dotenv().unwrap();

    let dir = env::var("PROJECTS_DIR").unwrap();

    let project = "test entries";

    let dal = DAL::new(dir);

    dal.create_project(project).await.unwrap();

    let p = dal.project(project).await.unwrap();

    assert_eq!(p.entries().await.unwrap(), vec![]);

    let e = CreateEntry {
      title: "x".to_owned(),
      body: "lorem ipsum".to_owned(),
      published: false,
    };

    assert_eq!(p.create_entry(e).await.unwrap(), 1);

    let e_expected = Entry {
      id: 1,
      title: "x".to_owned(),
      body: "lorem ipsum".to_owned(),
      published: false,
    };

    assert_eq!(p.entries().await.unwrap(), vec![e_expected]);

    p.delete_entry(1).await.unwrap();

    assert_eq!(p.entries().await.unwrap(), vec![]);

    dal.delete_project(project).unwrap();
  }
}