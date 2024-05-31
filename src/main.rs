use std::collections::HashMap;

use anyhow::Context;
use rust_tradier::data::{run_async, Handler};
use series_store::{SeriesReader, SeriesWriter, Topic};
use shared_types::{convert::serialize_timestamp, EventId, StdoutLogger, UtcDateTime};

use serde_json::{self, Value};

struct TradierHandler {
    next_event_ids: HashMap<Topic, EventId>,
    writer: SeriesWriter,
}

impl TradierHandler {
    fn new(next_event_ids: HashMap<Topic, EventId>) -> Self {
        let writer: SeriesWriter = SeriesWriter::new();
        Self {
            next_event_ids,
            writer,
        }
    }

    async fn sendit(&mut self, timestamp: &UtcDateTime, data: &str) -> anyhow::Result<()> {
        // println!("event id: {}, thread id: {:?}", event_id, std::thread::current());

        let v: Value = serde_json::from_str::<Value>(data)?;
        let event_type = v["type"].as_str().with_context(|| format!("Could not get event type from event: {}", data))?;
        let symbol = v["symbol"].as_str().with_context(|| format!("Could not get symbol from event: {}", data))?;
        let ts = serialize_timestamp(timestamp);
        let topic = Topic::new("raw", symbol, event_type);

        let event_id = *self.next_event_ids.get(&topic).unwrap_or_else(|| {
            println!("WARNING: event_id not found for topic: {}", topic);
            // TODO: go get latest for this topic?
            &1
        });
        self.writer.write(event_id, &topic, "key", ts, data).await.with_context(|| "Error writing to store")?;

        self.next_event_ids.insert(topic, event_id + 1);
        Ok(())
    }
}

impl Handler<String> for TradierHandler {
    async fn on_data_async(&mut self, timestamp: UtcDateTime, data: String) {
        let _ = self.sendit(&timestamp, &data).await.map_err(|e| {
            println!("{}: Error {} processing event: {}", timestamp, e, data);
        });
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let logger = StdoutLogger::boxed();
    let mut reader = SeriesReader::new(logger)?;
    let next_event_ids = reader.calc_next_event_ids()?;

    println!("Reading from tradier and writing to series-store");
    let h = TradierHandler::new(next_event_ids);
    run_async(h, &series_store::SYMBOLS).await;

    println!("Exiting");
    Ok(())
}
