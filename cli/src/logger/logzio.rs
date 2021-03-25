use chrono::Utc;
use hyper::{client::Client, Body, Request};
use log::{Level, Metadata, Record};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use tokio;

#[derive(Deserialize, Serialize, Debug)]
pub struct LogzIO;

impl log::Log for LogzIO {
  fn enabled(&self, metadata: &Metadata) -> bool {
    metadata.level() <= Level::Info
  }

  fn log(&self, record: &Record) {
    let utc_time = Utc::now().format("%FT%T%.3fZ");
    let env = env::var("ALAN_TECH_ENV").unwrap_or("production".to_string());
    let cluster = record.target(); // If no target defined default to `anycloud`
    let token = match env.as_str() {
      "local" => "ZERXpCvywsOBtNOXrqIzfpLiOnEXKXhb",
      "staging" => "ZERXpCvywsOBtNOXrqIzfpLiOnEXKXhb",
      "production" => "ZERXpCvywsOBtNOXrqIzfpLiOnEXKXhb",
      _ => "",
    };
    let url = format!(
      "https://listener.logz.io:8071/?token={}&type=anycloud",
      token
    );
    if self.enabled(record.metadata()) {
      let client = Client::builder().build::<_, Body>(hyper_tls::HttpsConnector::new());
      let req = Request::post(url).body(
        json!({
          "utc_time": utc_time.to_string(),
          "level": record.level(),
          "env": env,
          "cluster": cluster,
          "message": record.args(),
        })
        .to_string()
        .into(),
      );
      let req = match req {
        Ok(req) => req,
        Err(e) => {
          eprintln!("Error creating LogzIO request: {}", e);
          return;
        }
      };
      tokio::task::spawn(async move {
        let _res = client.request(req).await;
      });
    }
  }

  fn flush(&self) {}
}
