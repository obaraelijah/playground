use playground::{MyActor, Ping};

use actix::Actor;

#[actix_rt::test]
async fn test() {
  let addr = MyActor { count: 10 }.start();
  
  let res = addr.send(Ping(10)).await;
  
  assert_eq!(res.unwrap(), 20);
}