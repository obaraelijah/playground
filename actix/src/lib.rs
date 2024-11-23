use actix::{Actor, Context, Handler, Message, Recipient};

use jonases_tracing_util::tracing::{event, Level};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Ping(pub usize);

pub struct MyActor {
  id: usize,
  subscribers: Vec<Recipient<Ping>>,
}

impl MyActor {
  pub fn new(id: usize, subscribers: Vec<Recipient<Ping>>) -> Self {
    Self { id, subscribers }
  }
}

impl Actor for MyActor {
  type Context = Context<Self>;
}

impl Handler<Ping> for MyActor {
  type Result = ();

  fn handle(&mut self, msg: Ping, _ctx: &mut Context<Self>) {
    event!(
      Level::ERROR,
      "Actor: {} just got pinged from {}",
      self.id,
      msg.0
    );

    for s in &self.subscribers {
      s.do_send(Ping(self.id));
    }
  }
}
