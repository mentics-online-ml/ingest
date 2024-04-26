use anyhow::bail;
use chrono::NaiveDateTime;
use rust_tradier::data::{run_async, Handler};
use series_store::{SeriesReader, SeriesWriter};
use shared_types::{EventId, Features};


const SYMBOL: &str = "SPY";

struct TradierHandler {
    next_event_id: EventId,
    writer: SeriesWriter,
}

impl TradierHandler {
    fn new(starting_counter: u64) -> Self {
        let writer: SeriesWriter = SeriesWriter::new();
        Self {
            next_event_id: starting_counter,
            writer,
        }
    }
}

impl Handler<String> for TradierHandler {
    fn on_data(&mut self, timestamp: NaiveDateTime, data: String) {
        let event_id = self.next_event_id;
        println!("event id: {}, thread id: {:?}", event_id, std::thread::current());

        let features = Features::try_from(event_id, &data);
        // TODO: handle error case properly
        // TODO: check for different types like quote
        if features.is_err() {
            println!("Skipping message: {}", data);
            return;
        }
        let features = features.unwrap();

        // TODO: better error handling
        match self.writer.write_raw(
            SYMBOL,
            event_id,
            timestamp.timestamp_millis(),
            &data,
        ) {
            Ok(_) => (),
            Err(e) => {
                println!("Error writing raw to store: {:?}", e);
                return;
            }
        }

        // TODO: better error handling
        match self.writer.write_features(
            SYMBOL,
            event_id,
            timestamp.timestamp_millis(),
            &features,
        ) {
            Ok(_) => (),
            Err(e) => {
                println!("Error writing features to store: {:?}", e);
                return;
            }
        }

        self.next_event_id = event_id + 1;
    }
}

#[tokio::main]
async fn main() {
    let starting_counter = initial_counter().unwrap();

    println!("Reading from tradier and writing to series-store");
    let h = TradierHandler::new(starting_counter);
    run_async(h, SYMBOL).await;

    println!("Exiting");
}

fn initial_counter() -> anyhow::Result<u64> {
    let reader = SeriesReader::new()?;
    let ids = reader.try_most_recent_event_ids()?;
    println!("High watermarks {:?}", ids);

    if ids.raw == ids.event {
        Ok(ids.raw)
    } else {
        bail!("Unhandled id mismatch {} != {}", ids.raw, ids.event)
    }
}
