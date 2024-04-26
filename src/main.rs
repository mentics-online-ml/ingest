use std::collections::HashMap;
use std::env;
use std::time::Duration;
use chrono::NaiveDateTime;
use convert::{EventId, Features, EVENT_ID_FIELD};
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::error::KafkaError;
use rdkafka::message::{BorrowedMessage, Header, Headers, OwnedHeaders};
use rdkafka::{Message, Offset, TopicPartitionList};
use rust_tradier::data::{Handler, run_async};

use rdkafka::config::{ClientConfig, FromClientConfig};
use rdkafka::producer::{FutureProducer, FutureRecord};

const SYMBOL: &str = "SPY";

struct TradierHandler {
    last_event_id: EventId,
    topic_raw: String,
    topic_features: String,
    producer: FutureProducer
}

impl TradierHandler {
    fn new(brokers:&str, topic_raw:String, topic_features:String, starting_counter: u64) -> Self {
        let producer: FutureProducer = create_producer(brokers);
        Self { last_event_id:starting_counter, producer, topic_raw, topic_features }
    }
}

impl Handler<String> for TradierHandler {
    fn on_data(&mut self, timestamp:NaiveDateTime, data:String) {
        let event_id = self.last_event_id + 1;

        let headers = OwnedHeaders::new()
            .insert(Header { key: EVENT_ID_FIELD, value: Some(&event_id.to_string()) });

        let rec = FutureRecord::to(&self.topic_raw)
            .key(SYMBOL)
            .timestamp(timestamp.timestamp_millis())
            .headers(headers.clone())
            .payload(&data);
        self.producer.send_result(rec).expect("send_result expect 1");

        let features = Features::try_from(event_id, &data);
        // TODO: handle error case properly
        // TODO: check for different types like quote
        if features.is_err() {
            println!("Skipping message: {}", data);
            return;
        }
        let features = features.unwrap();

        let serialized = features.serialize();
        let rec = FutureRecord::to(&self.topic_features)
            .key(SYMBOL)
            .timestamp(timestamp.timestamp_millis())
            .headers(headers)
            .payload(&serialized);
        self.producer.send_result(rec).expect("send_result expect 1");

        self.last_event_id = event_id;
    }
}

#[tokio::main]
async fn main() {
    let topic_raw:String = env::var("TOPIC_RAW").unwrap();
    let topic_features:String = env::var("TOPIC_FEATURES").unwrap();
    let brokers:String = env::var("REDPANDA_ENDPOINT").unwrap();

    let starting_counter = establish_counter(&brokers, &topic_raw, &topic_features);

    println!("Reading from tradier and writing to redpanda topic {topic_raw}");

    let h = TradierHandler::new(&brokers, topic_raw, topic_features, starting_counter);

    // let h = Test { data: "none yet".to_string() };
    run_async(h, SYMBOL).await;

    // let consumer: StreamConsumer = ClientConfig::new()
    //     .set("bootstrap.servers", brokers)
    //     .set("session.timeout.ms", "6000")
    //     .set("enable.auto.commit", "false")
    //     .set("group.id", "rust-rdkafka-roundtrip-example")
    //     .create()
    //     .expect("Consumer creation failed");

    // consumer.subscribe(&[&topic])
    //     .expect("Can't subscribe to specified topic");


    // let message = consumer.recv().await.unwrap();
    // print!("Consumed: {:?}", message);


    // let producer = RedpandaBuilder::new()
    //     .set_bootstrap_servers("localhost:8021")
    //     // .enable_idempotence()
    //     .build_producer().unwrap();

    // let rec = RedpandaRecord::new("test-topic", None, "test message from rust".as_bytes().to_vec(), None);
    // producer.send_result(&rec).unwrap();
    // producer.send("test", "test message from rust".to_string()).unwrap();

    println!("End");
}

fn get_header<'a>(msg: &'a Option<Result<BorrowedMessage, KafkaError>>, key:&str) -> Option<&'a str> {
    match msg {
        Some(Ok(m)) => {
            m.headers().map(|headers|
                headers.iter().find(|h| h.key == key).map(|h| h.value.map(|h| std::str::from_utf8(h).ok()))
            ).flatten().flatten().flatten()
        }
        _ => return None
    }
}

fn establish_counter(brokers: &str, topic1: &str, topic2: &str) -> u64 {
    let consumer: BaseConsumer = create_consumer(brokers);
    // consumer.subscribe(&[topic1]).unwrap();
    let hm = HashMap::from([((topic1.to_string(), 0), Offset::OffsetTail(1))]);
    let tpl = TopicPartitionList::from_topic_map(&hm).unwrap();
    let res = consumer.assign(&tpl).unwrap();
    println!("Assign result: {:?}", res);
    // let res = consumer.seek(topic1, 0, Offset::OffsetTail(1), Duration::from_millis(1000));
    println!("Seek result: {:?}", res);
    let msg = consumer.poll(Duration::from_millis(1000));
    if let Some(Ok(m)) = &msg {
        println!("First payload on topic {} with offset {}: {}", topic1, m.offset(), std::str::from_utf8(m.payload().unwrap()).unwrap());
        return m.offset() as u64;
    }
    // println!("Latest message on topic {}: {:?}", topic1, msg);
    let result = get_header(&msg, EVENT_ID_FIELD).map(
        |v| v.parse::<u64>().unwrap_or(1)
    ).unwrap_or(1);
    println!("Found counter: {result}");
    return result;
    // TODO: handle other topic to make sure they both match
}

fn create_producer<R>(brokers:&str) -> R where R: FromClientConfig {
    ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error")
}

fn create_consumer<R>(brokers:&str) -> R where R: FromClientConfig {
    ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("group.id", "default_group")
        // .set("enable.auto.commit", "false")
        // .set("auto.offset.reset", "latest")
        .create()
        .expect("Consumer creation error")
}
