use kafka::producer::{Producer, Record, RequiredAcks};
use serde_json;


const ACCOUNTS_TOPIC_NAME: &str = "accounts";


pub struct KafkaClient {
    producer: Producer
}

impl KafkaClient {
    pub fn new(hosts: Vec<String>) -> Self {
        let producer = Producer::from_hosts(hosts)
                .with_required_acks(RequiredAcks::One)
            .create()
            .unwrap();

        Self {
            producer
        }
    }

    pub async fn produce(&mut self, data: &Option<serde_json::value::Value>) {
        let stringified_json: String = serde_json::to_string(data).ok().unwrap();
        let record = Record::from_value(
            ACCOUNTS_TOPIC_NAME, 
            stringified_json.as_bytes()
        );
        match self.producer.send(&record) {
            Ok(()) => println!("Message {} sent successfully", stringified_json),
            Err(err) => eprintln!("Error sending message: {}", err),
        }
    }
}
