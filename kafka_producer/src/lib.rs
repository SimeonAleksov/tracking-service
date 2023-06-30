use std::collections::hash_map::Values;
use kafka::producer::{Producer, Record};
use std::str;


const ACCOUNTS_TOPIC_NAME: String = "accounts".parse().unwrap();

async fn produce(data: String) {
    let hosts = vec!["localhost:9092".to_owned()];

    let mut producer: Producer = Producer::from_hosts(hosts)
        .create()
        .unwrap();

    let buf = format!("{data}");
    producer.send(
        &Record::from_value(
            &*ACCOUNTS_TOPIC_NAME.to_owned(),
            buf.as_bytes()))
        .unwrap();
    println!("Sent {data}")
}
