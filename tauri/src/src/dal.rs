use anyhow::anyhow;

use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

use std::fs;

#[derive(Clone)]
pub struct DAL {
  dir: String,
}

impl DAL {
  pub fn new<T: AsRef<str>>(dir: T) -> Self {
    Self {
      dir: dir.as_ref().to_owned(),
    }
  }

  async fn create_project<T: AsRef<str>>(
    &self,
    name: T,
  ) -> anyhow::Result<()> {
    let file_name = format!("{}/{}.db", self.dir, name.as_ref());

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

    Project::create(&db).await?;

    Ok(())
  }

  fn delete_project<T: AsRef<str>>(
    &self,
    name: T,
  ) -> anyhow::Result<()> {
    let file_name = format!("{}/{}.db", self.dir, name.as_ref());
    Ok(fs::remove_file(file_name)?)
  }

  fn list_projects(&self) -> anyhow::Result<Vec<String>> {
    let mut projects = Vec::new();

    for entry in fs::read_dir(&self.dir)? {
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
}

struct Project {
  pool: SqlitePool,
}

impl Project {
  async fn open(db: &str) -> anyhow::Result<Self> {
    let pool = SqlitePoolOptions::new()
      .max_connections(1)
      .connect(&db)
      .await?;

    Ok(Self { pool })
  }

  async fn create(db: &str) -> anyhow::Result<Self> {
    let instance = Self::open(db).await?;
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

    let dal = DAL::new(&dir);

    assert_eq!(dal.list_projects().unwrap(), Vec::<String>::new());

    // add a project

    dal.create_project("test 1").await.unwrap();

    assert_eq!(
      dal.list_projects().unwrap(),
      vec!["test 1.db".to_owned()],
    );

    // add random file, make sure it is excluded from test

    fs::File::create(format!("{}/not a db.txt", dir)).unwrap();

    assert_eq!(
      dal.list_projects().unwrap(),
      vec!["test 1.db".to_owned()],
    );

    // remove random file again

    fs::remove_file(format!("{}/not a db.txt", dir)).unwrap();

    // delete project created above

    dal.delete_project("test 1").unwrap();

    assert_eq!(dal.list_projects().unwrap(), Vec::<String>::new());
  }
}