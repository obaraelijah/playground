
use actix::{Actor, Context, Handler, Message};

#[derive(Message)]
#[rtype(result = "usize")]
pub struct Ping(pub usize);

pub struct MyActor {
  pub count: usize,
}

impl Actor for MyActor {
  type Context = Context<Self>;
}

impl Handler<Ping> for MyActor {
  type Result = usize;
  
  fn handle(
    &mut self,
    msg: Ping,
    _ctx: &mut Context<Self>,
  ) -> Self::Result {
    self.count += msg.0;
    self.count
  }
}