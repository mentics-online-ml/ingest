use anyhow::bail;
use chrono::NaiveDateTime;
use rust_tradier::data::{run_async, Handler};
use series_store::{SeriesReader, SeriesWriter};
use shared_types::{Event, EventId, Logger, StdoutLogger};


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

        let event = Event::try_from(event_id, &data);
        // TODO: handle error case properly
        // TODO: check for different types like quote
        if event.is_err() {
            println!("Skipping message: {}", data);
            return;
        }
        let event = event.unwrap();

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
        match self.writer.write_event(
            SYMBOL,
            event_id,
            timestamp.timestamp_millis(),
            &event,
        ) {
            Ok(_) => (),
            Err(e) => {
                println!("Error writing event to store: {:?}", e);
                return;
            }
        }

        self.next_event_id = event_id + 1;
    }
}

#[tokio::main]
async fn main() {
    let logger = StdoutLogger::boxed();
    let starting_counter = initial_counter(logger).unwrap();

    println!("Reading from tradier and writing to series-store");
    let h = TradierHandler::new(starting_counter);
    run_async(h, SYMBOL).await;

    println!("Exiting");
}

fn initial_counter(logger: Box<dyn Logger>) -> anyhow::Result<u64> {
    let reader = SeriesReader::new(logger)?;
    let ids = reader.try_most_recent_event_ids()?;
    println!("High watermarks {:?}", ids);

    if ids.raw == ids.event {
        Ok(ids.raw)
    } else {
        bail!("Unhandled id mismatch {} != {}", ids.raw, ids.event)
    }
}
