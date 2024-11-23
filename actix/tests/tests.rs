use playground::{MyActor, Ping};

use actix::Actor;

#[allow(unused_imports)]
use jonases_tracing_util::tracing::{event, Level};

#[actix_rt::test]
async fn test() {
  jonases_tracing_util::init_logger();

  let addr4 = MyActor::new(4, vec![]).start();

  let addr3 = MyActor::new(3, vec![addr4.recipient()]).start();

  let addr2 =
    MyActor::new(2, vec![addr3.clone().recipient()]).start();

  let addr1 =
    MyActor::new(1, vec![addr2.recipient(), addr3.recipient()])
      .start();

  addr1.send(Ping(0)).await.unwrap();
}
