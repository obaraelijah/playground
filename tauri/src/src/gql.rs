use crate::api::Entry;
use crate::dal::DAL;

use async_graphql::{
  EmptySubscription, Object, Request, Response, Schema as GQLSchema,
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
  async fn projects(&self) -> Vec<String> {
    self.dal.projects().unwrap()
  }

  async fn entries(&self, project: String) -> Vec<Entry> {
    self
      .dal
      .project(&project)
      .await
      .unwrap()
      .entries()
      .await
      .unwrap()
  }

  async fn entry(&self, project: String, id: u32) -> Entry {
    self
      .dal
      .project(&project)
      .await
      .unwrap()
      .entry(id)
      .await
      .unwrap()
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
  async fn create_project(&self, project: String) -> bool {
    self.dal.create_project(&project).await.unwrap();
    true
  }

  async fn delete_project(&self, project: String) -> bool {
    self.dal.delete_project(&project).unwrap();
    true
  }

  async fn create_entry(&self, project: String, e: Entry) -> bool {
    self
      .dal
      .project(&project)
      .await
      .unwrap()
      .create_entry(e)
      .await
      .unwrap();

    true
  }

  async fn delete_entry(&self, project: String, id: u32) -> bool {
    self
      .dal
      .project(&project)
      .await
      .unwrap()
      .delete_entry(id)
      .await
      .unwrap();

    true
  }
}