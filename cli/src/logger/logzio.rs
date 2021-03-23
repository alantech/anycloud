use chrono::{DateTime, Local, Utc};
use hyper::{client::Client, Body, Request};
use log::{Level, Metadata, Record};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio;

#[derive(Deserialize, Serialize, Debug)]
pub struct LogzIO;

impl log::Log for LogzIO {
  fn enabled(&self, metadata: &Metadata) -> bool {
    metadata.level() <= Level::Info
  }

  fn log(&self, record: &Record) {
    let local_time = Local::now();
    let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
    let url = "https://listener.logz.io:8071/?token=ZERXpCvywsOBtNOXrqIzfpLiOnEXKXhb&type=anycloud";
    if self.enabled(record.metadata()) {
      let client = Client::builder().build::<_, Body>(hyper_tls::HttpsConnector::new());
      let req = Request::post(url)
        .body(json!({
          "level": record.level(),
          "message": format!("{} | {} | {}", utc_time, record.level(), record.args()),
        }).to_string().into());
      let req = match req {
        Ok(req) => req,
        Err(e) => {
          eprintln!("Error creating LogzIO request: {}", e);
          return ();
        }
      };
      tokio::task::spawn(async move {
        let _res = client.request(req).await;
      });
    }
  }

  fn flush(&self) {}
}
