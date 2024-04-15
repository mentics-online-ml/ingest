use chrono::NaiveDateTime;
use rust_tradier::data::{Handler, run_async};
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
    const topic:&str = "spy";
    const brokers:&str = "redpanda-0.redpanda.rpanda.svc.cluster.local.:9093";

    println!("Reading from tradier and writing to redpanda topic {topic}");

    // ~/rpk -X brokers=redpanda-0.redpanda.rpanda.svc.cluster.local.:9093 -X tls.enabled=true -X tls.insecure_skip_verify=true topic list
    // redpanda-0.redpanda.rpanda.svc.cluster.local

    struct MyHandler {
        msg_counter:u64,
        producer:FutureProducer
    }
    impl MyHandler {
        fn new() -> Self {
            let producer: FutureProducer = ClientConfig::new()
                .set("bootstrap.servers", brokers)
                .set("message.timeout.ms", "5000")
                .create()
                .expect("Producer creation error");
            Self { msg_counter:0, producer }
        }
    }
    impl Handler<String> for MyHandler {
        fn on_data(&mut self, timestamp:NaiveDateTime, data:String) {
            self.msg_counter += 1;
            let rec = FutureRecord::to("spy")
                .key("blue")
                .payload(&data);
                // .timestamp(now());
            self.producer.send_result(rec).expect("send_result expect 1");
                // .await
                // .unwrap()
                // .unwrap();

            // println!("MyHandler::on_data received: {:?}", data);
        }
    }

    let h = MyHandler::new();

    // let h = Test { data: "none yet".to_string() };
    run_async(h).await;

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
