use crate::api::{CreateEntry, Entry};
use crate::dal::DAL;

use async_graphql::{
  EmptySubscription, Object, Request, Response, Result,
  Schema as GQLSchema,
};

pub struct Schema {
  inner: GQLSchema<Query, Mutation, EmptySubscription>,
}

impl Schema {
  pub fn new(dal: DAL) -> Self {
    let query = Query::new(dal.clone());
    let mutation = Mutation::new(dal.clone());

    let schema =
      GQLSchema::build(query, mutation, EmptySubscription).finish();

    Self { inner: schema }
  }

  pub async fn execute<T: Into<Request>>(
    &self,
    request: T,
  ) -> Response {
    self.inner.execute(request).await
  }
}

struct Query {
  dal: DAL,
}

impl Query {
  fn new(dal: DAL) -> Self {
    Self { dal }
  }
}

#[Object]
impl Query {
  async fn projects(&self) -> Result<Vec<String>> {
    Ok(self.dal.projects()?)
  }

  async fn entries(&self, project: String) -> Result<Vec<Entry>> {
    Ok(self.dal.project(&project).await?.entries().await?)
  }

  async fn entry(&self, project: String, id: u32) -> Result<Entry> {
    Ok(self.dal.project(&project).await?.entry(id).await?)
  }
}

struct Mutation {
  dal: DAL,
}

impl Mutation {
  fn new(dal: DAL) -> Self {
    Self { dal }
  }
}

#[Object]
impl Mutation {
  async fn create_project(&self, project: String) -> Result<bool> {
    self.dal.create_project(&project).await?;
    Ok(true)
  }

  async fn delete_project(&self, project: String) -> Result<bool> {
    self.dal.delete_project(&project)?;
    Ok(true)
  }

  async fn create_entry(
    &self,
    project: String,
    entry: CreateEntry,
  ) -> Result<u32> {
    Ok(
      self
        .dal
        .project(&project)
        .await?
        .create_entry(entry)
        .await?,
    )
  }

  async fn delete_entry(
    &self,
    project: String,
    id: u32,
  ) -> Result<bool> {
    self.dal.project(&project).await?.delete_entry(id).await?;
    Ok(true)
  }
}

#[cfg(test)]
mod tests {
  use serde::Deserialize;

  use async_graphql::from_value;

  use std::env;

  use crate::api::Entry;
  use crate::dal::DAL;

  use super::Schema;

  #[derive(Deserialize)]
  #[serde(rename_all = "camelCase")]
  struct Q {
    projects: Option<Vec<String>>,
    entries: Option<Vec<Entry>>,
    entry: Option<Entry>,
    create_entry: Option<u32>,
  }

  #[tokio::test]
  async fn test_projects() {
    dotenv::dotenv().unwrap();

    let dir = env::var("PROJECTS_DIR").unwrap();

    let dal = DAL::new(dir);

    let s = Schema::new(dal);

    let v = s
      .execute(
        r#"
        {
          projects
        }
      "#,
      )
      .await
      .into_result()
      .unwrap();

    let q: Q = from_value(v.data).unwrap();

    assert_eq!(q.projects.unwrap(), Vec::<String>::new());

    s.execute(
      r#"
        mutation {
          createProject(project: "test gql projects")
        }
      "#,
    )
    .await
    .into_result()
    .unwrap();

    let v = s
      .execute(
        r#"
        {
          projects
        }
      "#,
      )
      .await
      .into_result()
      .unwrap();

    let q: Q = from_value(v.data).unwrap();

    assert_eq!(
      q.projects.unwrap(),
      vec!["test gql projects".to_owned()]
    );

    s.execute(
      r#"
        mutation {
          deleteProject(project: "test gql projects")
        }
      "#,
    )
    .await
    .into_result()
    .unwrap();

    let v = s
      .execute(
        r#"
        {
          projects
        }
      "#,
      )
      .await
      .into_result()
      .unwrap();

    let q: Q = from_value(v.data).unwrap();

    assert_eq!(q.projects.unwrap(), Vec::<String>::new());
  }

  #[tokio::test]
  async fn test_entries() {
    let dir = env::var("PROJECTS_DIR").unwrap();

    let dal = DAL::new(dir);

    let s = Schema::new(dal);

    let entry_expected = Entry {
      id: 1,
      title: "x".to_owned(),
      body: "lorem ipsum".to_owned(),
      published: false,
    };

    s.execute(
      r#"
        mutation {
          createProject(project: "test gql entries")
        }
      "#,
    )
    .await
    .into_result()
    .unwrap();

    let v = s
      .execute(
        r#"
        {
          entries(project: "test gql entries") {
            id
            title
            body
            published
          }
        }
      "#,
      )
      .await
      .into_result()
      .unwrap();

    let q: Q = from_value(v.data).unwrap();

    assert_eq!(q.entries.unwrap(), Vec::<Entry>::new());

    let v = s
      .execute(
        r#"
        mutation {
          createEntry(
            project: "test gql entries",
            entry: {
              title: "x"
              body: "lorem ipsum"
              published: false
            }
          )
        }
      "#,
      )
      .await
      .into_result()
      .unwrap();

    println!("{:?}", v.data);

    let q: Q = from_value(v.data).unwrap();

    assert_eq!(q.create_entry.unwrap(), 1);

    let v = s
      .execute(
        r#"
        {
          entries(project: "test gql entries") {
            id
            title
            body
            published
          }
        }
      "#,
      )
      .await
      .into_result()
      .unwrap();

    let q: Q = from_value(v.data).unwrap();

    assert_eq!(q.entries.unwrap(), vec![entry_expected.clone()]);

    let v = s
      .execute(
        r#"
        {
          entry(project: "test gql entries" id: 1) {
            id
            title
            body
            published
          }
        }
      "#,
      )
      .await
      .into_result()
      .unwrap();

    let q: Q = from_value(v.data).unwrap();

    assert_eq!(q.entry.unwrap(), entry_expected);

    s.execute(
      r#"
        mutation {
          deleteEntry(project: "test gql entries", id: 1)
        }
      "#,
    )
    .await
    .into_result()
    .unwrap();

    let v = s
      .execute(
        r#"
        {
          entries(project: "test gql entries") {
            id
            title
            body
            published
          }
        }
      "#,
      )
      .await
      .into_result()
      .unwrap();

    let q: Q = from_value(v.data).unwrap();

    assert_eq!(q.entries.unwrap(), Vec::<Entry>::new());

    s.execute(
      r#"
        mutation {
          deleteProject(project: "test gql entries")
        }
      "#,
    )
    .await
    .into_result()
    .unwrap();
  }
}