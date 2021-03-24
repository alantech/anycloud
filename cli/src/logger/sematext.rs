use chrono::Utc;
use elasticsearch::{http::transport::Transport, Elasticsearch, IndexParts};
use log::{Level, Metadata, Record};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
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
    let utc_time = Utc::now().format("%FT%T%.3fZ");
    let env = env::var("ALAN_TECH_ENV").unwrap_or("production".to_string());
    let cluster = record.target(); // If no target defined default to `anycloud`
    let token = match env.as_str() {
      "local" => "f3c3fe7c-9689-470c-98c6-bc60e9b9649d",
      "staging" => "f3c3fe7c-9689-470c-98c6-bc60e9b9649d",
      "production" => "f3c3fe7c-9689-470c-98c6-bc60e9b9649d",
      _ => "",
    };
    if self.enabled(record.metadata()) {
      let future = ES_CLIENT
        .index(IndexParts::Index(token))
        .body(json!({
          "utc_time": utc_time.to_string(),
          "level": record.level(),
          "env": env,
          "cluster": cluster,
          "message": record.args(),
        }))
        .send();
      tokio::task::spawn(async move {
        let _res = future.await;
      });
    }
  }

  fn flush(&self) {}
}
