use chrono::NaiveDateTime;
use rust_tradier::data::{Handler, start};
// use redpanda::{builder::RedpandaBuilder, producer::RedpandaRecord};

use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use rdkafka::producer::{FutureProducer, FutureRecord};


struct Test {
    data:String
}

impl Handler<String> for Test {
    fn on_data(&mut self, timestamp:NaiveDateTime, data:String) {
        println!("Handler::on_data received: {:?}", data);
        self.data = data;
    }
}

#[tokio::main]
async fn main() {
    println!("Begin");

    let brokers = "localhost:9093";
    let topic = "test-topic";

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");


    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .set("group.id", "rust-rdkafka-roundtrip-example")
        .create()
        .expect("Consumer creation failed");

    consumer.subscribe(&[&topic]).unwrap();

    producer
        .send_result(
            FutureRecord::to(&topic)
                .key(&1.to_string())
                .payload("dummy")
                // .timestamp(now()),
        )
        .unwrap()
        .await
        .unwrap()
        .unwrap();



    let message = consumer.recv().await.unwrap();
    print!("Consumed: {:?}", message);



    // let producer = RedpandaBuilder::new()
    //     .set_bootstrap_servers("localhost:8021")
    //     // .enable_idempotence()
    //     .build_producer().unwrap();

    // let rec = RedpandaRecord::new("test-topic", None, "test message from rust".as_bytes().to_vec(), None);
    // producer.send_result(&rec).unwrap();
    // producer.send("test", "test message from rust".to_string()).unwrap();

    // let h = Test { data: "none yet".to_string() };
    // start(h);
    // std::thread::sleep(std::time::Duration::from_secs(4));
    // println!("End");
}
