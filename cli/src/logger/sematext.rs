use chrono::{DateTime, Local, Utc};
use elasticsearch::{http::transport::Transport, Elasticsearch, IndexParts};
use log::{Level, Metadata, Record};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio;

static ES_CLIENT: Lazy<Elasticsearch> = Lazy::new(|| {
  Elasticsearch::new(Transport::single_node("https://logsene-receiver.sematext.com").unwrap())
});

#[derive(Deserialize, Serialize, Debug)]
pub struct Sematext;

impl log::Log for Sematext {
  fn enabled(&self, metadata: &Metadata) -> bool {
    metadata.level() <= Level::Info
  }

  fn log(&self, record: &Record) {
    let local_time = Local::now();
    let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
    if self.enabled(record.metadata()) {
      let future = ES_CLIENT
        // TODO: make this key dynamic based on env
        .index(IndexParts::Index("f3c3fe7c-9689-470c-98c6-bc60e9b9649d"))
        .body(json!({
          "level": record.level(),
          "message": format!("{} | {} | {}", utc_time, record.level(), record.args()),
        }))
        .send();
      tokio::task::spawn(async move {
        let _res = future.await;
      });
    }
  }

  fn flush(&self) {}
}
