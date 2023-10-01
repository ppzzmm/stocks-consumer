extern crate stocks_consumer;

use kafka::consumer::{Consumer, FetchOffset};
use std::{str, env};
use stocks_consumer::buy_stocks;
use stocks_consumer::persistence::connection::create_connection_pool;
use stocks_consumer::{run_migrations};

fn main() {
  let pool = create_connection_pool();
  run_migrations(&mut pool.get().expect("Can't get DB connection"));
    #[allow(unused_assignments)]
    let mut url_kafka = "".to_string();
    match env::var("KAFKA_BROKER") {
        Ok(stream) => {
            url_kafka = format!("{}", stream);
        }
        Err(_e) => {
            url_kafka = "localhost:9092".to_string();
        }
    };
    let hosts = vec![url_kafka];
    let mut consumer =
       Consumer::from_hosts(hosts)
          .with_topic("topic-stocks".to_owned())
          .with_fallback_offset(FetchOffset::Earliest)
          .create()
          .unwrap();
    loop {
      for ms in consumer.poll().unwrap().iter() {
        for m in ms.messages() {
          println!("{:?}", str::from_utf8(m.value).unwrap());
          let parts = str::from_utf8(m.value).unwrap().split(",");
          let collection = parts.collect::<Vec<&str>>();
          if collection.len() == 3 {
            let symbol = collection[0];
            let shares = collection[1];
            let action = collection[2];
            buy_stocks(
              symbol.to_string(),
              shares.to_string(),
              action.to_string(),
              &mut pool.get().expect("Can't get DB connection"));
          }
        }
        let _ = consumer.consume_messageset(ms);
      }
      consumer.commit_consumed().unwrap();
    }
}