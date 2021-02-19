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
struct AWSCredentials {
  accessKeyId: String,
  secretAccessKey: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Serialize)]
struct AWSConfig {
  credentials: AWSCredentials,
  region: String,
  cloudProvider: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct App {
  id: String,
  url: String,
  cloudProvider: String,
  cloudAlias: String,
  version: String,
}

const CONFIG_NAME: &str = ".alan/deploy.json";
const CONFIG_SCHEMA: &str = "Please define a deploy config with the following schema: \n{
  \"cloudAlias\": {
    \"cloudProvider\": \"string\",
    \"region\": \"string\",
    \"credentials\": {
      \"accessKeyId\": \"string\",
      \"secretAccessKey\": \"string\",
    }
  },
  \"cloudAlias\": {
    ...
  }
}";
const HOW_TO_AWS: &str = "
To create an AWS access key follow this tutorial:\n\nhttps://aws.amazon.com/premiumsupport/knowledge-center/create-access-key/\n
Then enable programmatic access for the IAM user, and attach the built-in 'AdministratorAccess' policy to your IAM user.
";

fn get_config() -> HashMap<String, AWSConfig> {
  let home = std::env::var("HOME").unwrap();
  let file_name = &format!("{}/{}", home, CONFIG_NAME);
  let path = Path::new(file_name);
  let file = File::open(path);
  if let Err(err) = file {
    println!("Cannot access deploy config at {}. Error: {}", file_name, err);
    println!("{}", CONFIG_SCHEMA);
    println!("{}", HOW_TO_AWS);
    std::process::exit(1);
  }
  let reader = BufReader::new(file.unwrap());
  let config = from_reader(reader);
  if let Err(err) = config {
    println!("Invalid deploy config. Error: {}", err);
    println!("{}", CONFIG_SCHEMA);
    println!("{}", HOW_TO_AWS);
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

fn get_app_str(agz_file: &str) -> String {
  let path = Path::new(agz_file);
  if path.extension().is_none() || path.extension().unwrap() != "agz" {
    println!("Deploy failed. The provided file must be an .agz file");
    std::process::exit(1);
  }
  let app = read(agz_file).expect(&format!("Deploy failed parsing {}", agz_file));
  return base64::encode(app);
}

pub async fn terminate(cluster_id: &str) {
  let body = json!({
    "deployConfig": get_config(),
    "clusterId": cluster_id,
  });
  let sp = SpinnerBuilder::new(format!("Terminating app {} if it exists", cluster_id)).start();
  let resp = post_v1("terminate", body).await;
  let res = match resp {
    Ok(_) => format!("Terminated app {} succesfully!", cluster_id),
    Err(err) => format!("Failed to terminate app {}. Error: {}", cluster_id, err),
  };
  sp.message(res);
  sp.close();
}

pub async fn new(agz_file: &str, cloud_alias: &str) {
  let app_str = get_app_str(agz_file);
  let body = json!({
    "deployConfig": get_config(),
    "agzB64": app_str,
    "cloudAlias": cloud_alias,
  });
  let body = json!(body);
  let sp = SpinnerBuilder::new(format!("Creating new app in {}", cloud_alias)).start();
  let resp = post_v1("new", body).await;
  let res = match resp {
    Ok(cluster_id) => format!("Created app with id {} in {} succesfully!", cluster_id, cloud_alias),
    Err(err) => format!("Failed to create a new app in {}. Error: {}", cloud_alias, err),
  };
  sp.message(res);
  sp.close();
}

pub async fn upgrade(cluster_id: &str, agz_file: &str) {
  let app_str = get_app_str(agz_file);
  let body = json!({
    "deployConfig": get_config(),
    "clusterId": cluster_id,
    "agzB64": app_str,
  });
  let sp = SpinnerBuilder::new(format!("Upgrading app {} with {}", cluster_id, agz_file)).start();
  let resp = post_v1("upgrade", body).await;
  let res = match resp {
    Ok(_) => format!("Upgraded app {} succesfully!", cluster_id),
    Err(err) => format!("Failed to upgrade app {} with {}. Error: {}", cluster_id, agz_file, err),
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
    data.push(vec![&app.id, &app.url, &app.cloudProvider, &app.cloudAlias, &app.version]);
  }

  println!("Status of all apps deployed using the cloud credentials in ~/{}\n", CONFIG_NAME);
  ascii_table.print(data);
}
