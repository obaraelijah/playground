use anyhow::anyhow;

use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct DAL {
  path: PathBuf,
}

impl DAL {
  pub fn new<T: Into<PathBuf>>(path: T) -> Self {
    Self { path: path.into() }
  }

  fn project_path<P: AsRef<Path>>(&self, project: P) -> PathBuf {
    let mut path = self.path.clone();
    path.push(project);
    path
  }

  async fn create_project<P: AsRef<Path>>(
    &self,
    project: P,
  ) -> anyhow::Result<()> {
    let path = self.project_path(project);

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

    Project::create(path).await?;

    Ok(())
  }

  fn delete_project<P: AsRef<Path>>(
    &self,
    project: P,
  ) -> anyhow::Result<()> {
    Ok(fs::remove_file(self.project_path(project))?)
  }

  fn list_projects(&self) -> anyhow::Result<Vec<String>> {
    let mut projects = Vec::new();

    for entry in fs::read_dir(&self.path)? {
      projects.push(entry?.file_name().into_string().map_err(
        |_| anyhow!("couldn't parse file name into valid UTF-8"),
      )?);
    }

    Ok(projects)
  }
}

struct Project {
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
}

#[cfg(test)]
mod tests {
  use std::env;
  use std::fs;

  use super::DAL;

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
  async fn test_list_projects() {
    dotenv::dotenv().unwrap();

    let dir = env::var("PROJECTS_DIR").unwrap();

    let dal = DAL::new(dir);

    assert_eq!(dal.list_projects().unwrap(), Vec::<String>::new());

    // add a project

    dal.create_project("test 1").await.unwrap();

    assert_eq!(
      dal.list_projects().unwrap(),
      vec!["test 1".to_owned()],
    );

    // delete project

    dal.delete_project("test 1").unwrap();

    assert_eq!(dal.list_projects().unwrap(), Vec::<String>::new());
  }
}