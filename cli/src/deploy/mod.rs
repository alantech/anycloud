use hyper::{client::Client, Body, Request};
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, from_str, json, Value};
use spinner::SpinnerBuilder;

use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::fs::{File, read};
use std::io::BufReader;
use std::path::Path;

use ascii_table::{AsciiTable, Column, Align};
use base64;

const URL: &str = if cfg!(debug_assertions) {
  "http://localhost:8080"
} else {
  "https://deploy.alantechnologies.com"
};

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Serialize)]
pub struct AWSCredentials {
  accessKeyId: String,
  secretAccessKey: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Serialize)]
pub struct AWSConfig {
  credentials: AWSCredentials,
  region: String,
  cloudProvider: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct GCPCredentials {
  private_key: String,
  client_email: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Serialize)]
pub struct GCPConfig {
  credentials: GCPCredentials,
  region: String,
  cloudProvider: String,
  projectId: String,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(untagged)]
pub enum Config {
  GCP(GCPConfig),
  AWS(AWSConfig),
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct App {
  id: String,
  url: String,
  cloudProvider: String,
  deployName: String,
  version: String,
}

const CONFIG_NAME: &str = ".anycloud/deploy.json";
// TODO: Have a command to do this for users
const CONFIG_SETUP: &str = "To create valid Anycloud deploy configs follow the instructions at:\n\nhttps://alantech.gitbook.io/anycloud";

pub fn get_config() -> HashMap<String, Vec<Config>> {
  let home = std::env::var("HOME").unwrap();
  let file_name = &format!("{}/{}", home, CONFIG_NAME);
  let path = Path::new(file_name);
  let file = File::open(path);
  if let Err(err) = file {
    println!("Cannot access deploy config at {}. Error: {}", file_name, err);
    println!("{}", CONFIG_SETUP);
    std::process::exit(1);
  }
  let reader = BufReader::new(file.unwrap());
  let config = from_reader(reader);
  if let Err(err) = config {
    println!("Invalid deploy config. Error: {}", err);
    println!("{}", CONFIG_SETUP);
    std::process::exit(1);
  }
  config.unwrap()
}

pub async fn post_v1(endpoint: &str, body: Value) -> Result<String, Box<dyn Error>> {
  let client = Client::builder().build::<_, Body>(hyper_tls::HttpsConnector::new());
  let req = Request::post(format!("{}/v1/{}", URL, endpoint))
    .header("Content-Type", "application/json")
    .body(body.to_string().into())?;
  let mut resp = client.request(req).await?;
  let data = hyper::body::to_bytes(resp.body_mut()).await?;
  let data_str = String::from_utf8(data.to_vec())?;
  return if resp.status().is_success() {
    Ok(data_str)
  } else {
    Err(data_str.into())
  };
}

pub fn get_file_str(file: &str) -> String {
  let f = read(file).expect(&format!("Deploy failed parsing {}", file));
  return base64::encode(f);
}

pub async fn terminate(cluster_id: &str) {
  let body = json!({
    "deployConfig": get_config(),
    "clusterId": cluster_id,
  });
  let sp = SpinnerBuilder::new(format!("Terminating app {} if it exists", cluster_id)).start();
  let resp = post_v1("terminate", body).await;
  let res = match resp {
    Ok(_) => format!("Terminated app {} successfully!", cluster_id),
    Err(err) => format!("Failed to terminate app {}. Error: {}", cluster_id, err),
  };
  sp.message(res);
  sp.close();
}

pub async fn new(body: Value) {
  let sp = SpinnerBuilder::new(format!("Creating new app")).start();
  let resp = post_v1("new", body).await;
  let res = match resp {
    Ok(cluster_id) => format!("Created app with id {} successfully!", cluster_id),
    Err(err) => format!("Failed to create a new app. Error: {}", err),
  };
  sp.message(res);
  sp.close();
}

pub async fn upgrade(body: Value) {
  let sp = SpinnerBuilder::new(format!("Upgrading app")).start();
  let resp = post_v1("upgrade", body).await;
  let res = match resp {
    Ok(_) => format!("Upgraded app successfully!"),
    Err(err) => format!("Failed to upgrade app. Error: {}", err),
  };
  sp.message(res);
  sp.close();
}

pub async fn info() {
  let body = json!({
    "deployConfig": get_config(),
  });
  let resp = post_v1("info", body).await;
  if let Err(err) = resp {
    println!("Displaying status for apps failed with error: {}", err);
    std::process::exit(1);
  }
  let mut apps: Vec<App> = from_str(resp.unwrap().as_str()).unwrap();

  if apps.len() == 0 {
    println!("No apps deployed using the cloud credentials in {}", CONFIG_NAME);
    return;
  }

  let mut ascii_table = AsciiTable::default();
  ascii_table.max_width = 140;

  let mut column = Column::default();
  column.header = "App Id".into();
  column.align = Align::Left;
  ascii_table.columns.insert(0, column);

  let mut column = Column::default();
  column.header = "Url".into();
  column.align = Align::Left;
  ascii_table.columns.insert(1, column);

  let mut column = Column::default();
  column.header = "Cloud".into();
  column.align = Align::Left;
  ascii_table.columns.insert(2, column);

  let mut column = Column::default();
  column.header = "Cloud Alias".into();
  column.align = Align::Left;
  ascii_table.columns.insert(3, column);

  let mut column = Column::default();
  column.header = "Version".into();
  column.align = Align::Left;
  ascii_table.columns.insert(4, column);

  let mut data: Vec<Vec<&dyn Display>> = vec![];
  for app in &mut apps {
    data.push(vec![&app.id, &app.url, &app.cloudProvider, &app.deployName, &app.version]);
  }

  println!("Status of all apps deployed using the cloud credentials in ~/{}\n", CONFIG_NAME);
  ascii_table.print(data);
}
