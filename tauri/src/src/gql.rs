use crate::api::Entry;
use crate::dal::DAL;

use async_graphql::{
  connection::EmptyFields, EmptySubscription, Object, Request,
  Response, Result, Schema as GQLSchema,
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
    let entries = self.dal.project(&project).await?.entries().await?;

    Ok(entries)
  }

  async fn entry(&self, project: String, id: u32) -> Result<Entry> {
    let entry = self.dal.project(&project).await?.entry(id).await?;

    Ok(entry)
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
  async fn create_project(
    &self,
    project: String,
  ) -> Result<EmptyFields> {
    self.dal.create_project(&project).await?;
    Ok(EmptyFields)
  }

  async fn delete_project(
    &self,
    project: String,
  ) -> Result<EmptyFields> {
    self.dal.delete_project(&project)?;
    Ok(EmptyFields)
  }

  async fn create_entry(
    &self,
    project: String,
    e: Entry,
  ) -> Result<EmptyFields> {
    self.dal.project(&project).await?.create_entry(e).await?;

    Ok(EmptyFields)
  }

  async fn delete_entry(
    &self,
    project: String,
    id: u32,
  ) -> Result<EmptyFields> {
    self.dal.project(&project).await?.delete_entry(id).await?;

    Ok(EmptyFields)
  }
}