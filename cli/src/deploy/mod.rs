use hyper::{Request, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, from_str, json, Value};
use spinner::SpinnerBuilder;

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::Display;
use std::fs::{read, File};
use std::io::BufReader;
use std::path::Path;

use ascii_table::{AsciiTable, Column};
use base64;
use log::error;

use crate::http::CLIENT;

const REQUEST_TIMEOUT: &str =
  "Operation is still in progress. It might take a few more minutes for \
  the cloud provider to finish up.";
const FORBIDDEN_OPERATION: &str =
  "Please review your credentials. Make sure you have follow all the \
  configuration steps: https://alantech.gitbook.io/anycloud/";
const NAME_CONFLICT: &str = "Another application with same App Id already exists.";

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct AWSCredentials {
  accessKeyId: String,
  secretAccessKey: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct GCPCredentials {
  privateKey: String,
  clientEmail: String,
  projectId: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct AzureCredentials {
  applicationId: String,
  secret: String,
  subscriptionId: String,
  directoryId: String,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Credentials {
  GCP(GCPCredentials),
  AWS(AWSCredentials),
  Azure(AzureCredentials),
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Serialize)]
pub struct CredentialsConfig {
  credentials: Credentials,
  cloudProvider: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Serialize)]
pub struct DeployConfig {
  credentials: String,
  region: String,
  vmType: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
  credentials: Credentials,
  region: String,
  cloudProvider: String,
  vmType: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct App {
  id: String,
  url: String,
  deployName: String,
  version: String,
  size: usize,
}

#[derive(Debug)]
pub enum PostV1Error {
  Timeout,
  Forbidden,
  Conflict,
  Other(Box<dyn Error>),
}

const DEPLOY_CONFIG_FILE: &str = "anycloud.json";
const CREDENTIALS_CONFIG_FILE: &str = ".anycloud/credentials.json";
// TODO: Have a command to do this for users
const CONFIG_SETUP: &str = "To create valid Anycloud deploy configs follow the instructions at:\n\nhttps://alantech.gitbook.io/anycloud";

fn get_credentials() -> HashMap<String, CredentialsConfig> {
  let home = std::env::var("HOME").unwrap();
  let file_name = &format!("{}/{}", home, CREDENTIALS_CONFIG_FILE);
  let path = Path::new(file_name);
  let file = File::open(path);
  if let Err(err) = file {
    error!("Cannot access credentials at {}. Error: {}", file_name, err);
    error!("{}", CONFIG_SETUP);
    std::process::exit(1);
  }
  let reader = BufReader::new(file.unwrap());
  let config = from_reader(reader);
  if let Err(err) = config {
    error!("Invalid credentials. Error: {}", err);
    error!("{}", CONFIG_SETUP);
    std::process::exit(1);
  }
  config.unwrap()
}

fn get_deploy_config() -> HashMap<String, Vec<DeployConfig>> {
  let home = std::env::var("PWD").unwrap();
  let file_name = &format!("{}/{}", home, DEPLOY_CONFIG_FILE);
  let path = Path::new(file_name);
  let file = File::open(path);
  if let Err(err) = file {
    error!(
      "Cannot access deploy config at {}. Error: {}",
      file_name, err
    );
    error!("{}", CONFIG_SETUP);
    std::process::exit(1);
  }
  let reader = BufReader::new(file.unwrap());
  let config = from_reader(reader);
  if let Err(err) = config {
    error!("Invalid deploy config. Error: {}", err);
    error!("{}", CONFIG_SETUP);
    std::process::exit(1);
  }
  config.unwrap()
}

// This method can be called as a binary by the end user in the CLI or as a library by the Alan daemon
// to send stats to the deploy service. We default to production so that it works as-is when it is used
// as a binary and we override the value it can have to our needs
fn get_url() -> &'static str {
  let env = std::env::var("ALAN_TECH_ENV").unwrap_or("production".to_string());
  match env.as_str() {
    "local" => "http://localhost:8080",
    "staging" => "https://deploy-staging.alantechnologies.com",
    _ => "https://deploy.alantechnologies.com",
  }
}

pub fn get_config() -> HashMap<String, Vec<Config>> {
  let anycloud_config = get_deploy_config();
  let cred_configs = get_credentials();
  let mut all_configs = HashMap::new();
  for (deploy_id, deploy_configs) in anycloud_config.into_iter() {
    let mut configs = Vec::new();
    for deploy_config in deploy_configs {
      let credentials = cred_configs
        .get(&deploy_config.credentials)
        .expect(&format!(
          "Credentials {} for deploy config {} not found in {}",
          &deploy_config.credentials, deploy_id, CREDENTIALS_CONFIG_FILE
        ));
      configs.push(Config {
        credentials: credentials.credentials.clone(),
        cloudProvider: credentials.cloudProvider.to_string(),
        region: deploy_config.region,
        vmType: deploy_config.vmType,
      });
    }
    all_configs.insert(deploy_id, configs);
  }
  all_configs
}

pub async fn post_v1(endpoint: &str, body: Value) -> Result<String, PostV1Error> {
  let url = get_url();
  let req = Request::post(format!("{}/v1/{}", url, endpoint))
    .header("Content-Type", "application/json")
    .body(body.to_string().into());
  let req = match req {
    Ok(req) => req,
    Err(e) => return Err(PostV1Error::Other(e.into())),
  };
  let resp = CLIENT.request(req).await;
  let mut resp = match resp {
    Ok(resp) => resp,
    Err(e) => return Err(PostV1Error::Other(e.into())),
  };
  let data = hyper::body::to_bytes(resp.body_mut()).await;
  let data = match data {
    Ok(data) => data,
    Err(e) => return Err(PostV1Error::Other(e.into())),
  };
  let data_str = String::from_utf8(data.to_vec());
  let data_str = match data_str {
    Ok(data_str) => data_str,
    Err(e) => return Err(PostV1Error::Other(e.into())),
  };
  return match resp.status() {
    st if st.is_success() => Ok(data_str),
    StatusCode::REQUEST_TIMEOUT => Err(PostV1Error::Timeout),
    StatusCode::FORBIDDEN => Err(PostV1Error::Forbidden),
    StatusCode::CONFLICT => Err(PostV1Error::Conflict),
    _ => Err(PostV1Error::Other(data_str.into())),
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
    Err(err) => match err {
      PostV1Error::Timeout => format!("{}", REQUEST_TIMEOUT),
      PostV1Error::Forbidden => format!("{}", FORBIDDEN_OPERATION),
      PostV1Error::Conflict => format!(
        "Failed to terminate app {}. Error: {}",
        cluster_id, NAME_CONFLICT
      ),
      PostV1Error::Other(err) => format!("Failed to terminate app {}. Error: {}", cluster_id, err),
    },
  };
  sp.message(res);
  sp.close();
}

pub async fn new(body: Value) {
  let sp = SpinnerBuilder::new(format!("Creating new app")).start();
  let resp = post_v1("new", body).await;
  let res = match resp {
    Ok(res) => format!("Created app with id {} successfully!", res),
    Err(err) => match err {
      PostV1Error::Timeout => format!("{}", REQUEST_TIMEOUT),
      PostV1Error::Forbidden => format!("{}", FORBIDDEN_OPERATION),
      PostV1Error::Conflict => format!("Failed to create a new app. Error: {}", NAME_CONFLICT),
      PostV1Error::Other(err) => format!("Failed to create a new app. Error: {}", err),
    },
  };
  sp.message(res);
  sp.close();
}

pub async fn upgrade(body: Value) {
  let sp = SpinnerBuilder::new(format!("Upgrading app")).start();
  let resp = post_v1("upgrade", body).await;
  let res = match resp {
    Ok(_) => format!("Upgraded app successfully!"),
    Err(err) => match err {
      PostV1Error::Timeout => format!("{}", REQUEST_TIMEOUT),
      PostV1Error::Forbidden => format!("{}", FORBIDDEN_OPERATION),
      PostV1Error::Conflict => format!("Failed to create a new app. Error: {}", NAME_CONFLICT),
      PostV1Error::Other(err) => format!("Failed to create a new app. Error: {}", err),
    },
  };
  sp.message(res);
  sp.close();
}

pub async fn info() {
  let body = json!({
    "deployConfig": get_config(),
  });
  let resp = post_v1("info", body).await;
  let resp = match &resp {
    Ok(resp) => resp,
    Err(err) => match err {
      PostV1Error::Timeout => {
        error!("{}", REQUEST_TIMEOUT);
        std::process::exit(1);
      }
      PostV1Error::Forbidden => {
        error!("{}", FORBIDDEN_OPERATION);
        std::process::exit(1);
      }
      PostV1Error::Conflict => {
        error!(
          "Displaying status for apps failed with error: {}",
          NAME_CONFLICT
        );
        std::process::exit(1);
      }
      PostV1Error::Other(err) => {
        error!("Displaying status for apps failed with error: {}", err);
        std::process::exit(1);
      }
    },
  };
  let mut apps: Vec<App> = from_str(resp).unwrap();

  if apps.len() == 0 {
    println!("No apps currently deployed");
    return;
  }

  let mut clusters = AsciiTable::default();
  clusters.max_width = 140;

  let column = Column {
    header: "App Id".into(),
    ..Column::default()
  };
  clusters.columns.insert(0, column);

  let column = Column {
    header: "Url".into(),
    ..Column::default()
  };
  clusters.columns.insert(1, column);

  let column = Column {
    header: "Deploy Config".into(),
    ..Column::default()
  };
  clusters.columns.insert(2, column);

  let column = Column {
    header: "Size".into(),
    ..Column::default()
  };
  clusters.columns.insert(3, column);

  let column = Column {
    header: "Version".into(),
    ..Column::default()
  };
  clusters.columns.insert(4, column);

  let mut deploy_names = HashSet::new();
  let mut data: Vec<Vec<&dyn Display>> = vec![];
  for app in &mut apps {
    deploy_names.insert(&app.deployName);
    data.push(vec![
      &app.id,
      &app.url,
      &app.deployName,
      &app.size,
      &app.version,
    ]);
  }

  println!("Status of all apps deployed:\n");
  clusters.print(data);

  let mut data: Vec<Vec<&dyn Display>> = vec![];
  let mut deploy = AsciiTable::default();
  deploy.max_width = 140;

  let column = Column {
    header: "Deploy Config".into(),
    ..Column::default()
  };
  deploy.columns.insert(0, column);

  let column = Column {
    header: "Credentials".into(),
    ..Column::default()
  };
  deploy.columns.insert(1, column);

  let column = Column {
    header: "Cloud Provider".into(),
    ..Column::default()
  };
  deploy.columns.insert(2, column);

  let column = Column {
    header: "Region".into(),
    ..Column::default()
  };
  deploy.columns.insert(3, column);

  let column = Column {
    header: "VM Type".into(),
    ..Column::default()
  };
  deploy.columns.insert(4, column);

  let deploy_configs = get_deploy_config();
  let credentials = get_credentials();
  for deploy_name in deploy_names {
    let cloud_configs = deploy_configs.get(&deploy_name.to_string()).unwrap();
    for (i, cloud_config) in cloud_configs.iter().enumerate() {
      let creds = credentials.get(&cloud_config.credentials).expect(&format!(
        "Credentials {} for deploy config {} not found in {}",
        &cloud_config.credentials, deploy_name, CREDENTIALS_CONFIG_FILE
      ));
      if i == 0 {
        data.push(vec![
          deploy_name,
          &cloud_config.credentials,
          &creds.cloudProvider,
          &cloud_config.region,
          &cloud_config.vmType,
        ])
      } else {
        data.push(vec![
          &"",
          &cloud_config.credentials,
          &creds.cloudProvider,
          &cloud_config.region,
          &cloud_config.vmType,
        ])
      };
    }
  }

  println!("\nDeployment configurations used:\n");
  deploy.print(data);
}
