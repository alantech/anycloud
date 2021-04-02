use hyper::{Request, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, from_str, json, Value};
use spinner::SpinnerBuilder;

use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use ascii_table::{AsciiTable, Column};

use crate::http::CLIENT;
use crate::oauth::{clear_token, get_token};
use crate::CLUSTER_ID;

pub const ALAN_VERSION: &'static str = env!("ALAN_VERSION");
const REQUEST_TIMEOUT: &str =
  "Operation is still in progress. It might take a few more minutes for \
  the cloud provider to finish up.";
const FORBIDDEN_OPERATION: &str =
  "Please review your credentials. Make sure you have follow all the \
  configuration steps: https://alantech.gitbook.io/anycloud/";
const NAME_CONFLICT: &str = "Another application with same app ID already exists.";
const UNAUTHORIZED_OPERATION: &str =
  "Invalid AnyCloud authentication credentials. Please retry and you will be asked to reauthenticate.";

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
pub struct CredentialsProfile {
  credentials: Credentials,
  cloudProvider: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Serialize)]
pub struct DeployProfile {
  credentialProfile: Option<String>,
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
  status: String,
  size: usize,
  cloudConfigs: Vec<Config>,
}

#[derive(Debug)]
pub enum PostV1Error {
  Timeout,
  Forbidden,
  Conflict,
  Unauthorized,
  Other(String),
}

const ANYCLOUD_FILE: &str = "anycloud.json";
const CREDENTIALS_FILE: &str = ".anycloud/credentials.json";
// TODO: Have a command to do this for users
const CONFIG_SETUP: &str = "To create valid Anycloud deploy configs follow the instructions at:\n\nhttps://alantech.gitbook.io/anycloud";

async fn get_cred_profiles() -> HashMap<String, CredentialsProfile> {
  let home = std::env::var("HOME").unwrap();
  let file_name = &format!("{}/{}", home, CREDENTIALS_FILE);
  let path = Path::new(file_name);
  let file = File::open(path);
  if let Err(err) = file {
    error!(
      "NO_CREDENTIALS_FILE",
      "Cannot access credentials at {}. Error: {}", file_name, err
    )
    .await;
    eprintln!("{}", CONFIG_SETUP); // Hint
    std::process::exit(1);
  }
  let reader = BufReader::new(file.unwrap());
  let config = from_reader(reader);
  if let Err(err) = config {
    error!(
      "INVALID_CREDENTIALS_FILE",
      "Invalid credentials. Error: {}", err
    )
    .await;
    eprintln!("{}", CONFIG_SETUP); // Hint
    std::process::exit(1);
  }
  config.unwrap()
}

async fn get_deploy_profile() -> HashMap<String, Vec<DeployProfile>> {
  let home = std::env::var("PWD").unwrap();
  let file_name = &format!("{}/{}", home, ANYCLOUD_FILE);
  let path = Path::new(file_name);
  let file = File::open(path);
  if let Err(err) = file {
    error!(
      "NO_ANYCLOUD_FILE",
      "Cannot access deploy config at {}. Error: {}", file_name, err
    )
    .await;
    eprintln!("{}", CONFIG_SETUP); // Hint
    std::process::exit(1);
  }
  let reader = BufReader::new(file.unwrap());
  let config = from_reader(reader);
  if let Err(err) = config {
    error!(
      "INVALID_ANYCLOUD_FILE",
      "Invalid deploy config. Error: {}", err
    )
    .await;
    eprintln!("{}", CONFIG_SETUP); // Hint
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

pub async fn get_config() -> HashMap<String, Vec<Config>> {
  let anycloud_prof = get_deploy_profile().await;
  let cred_profs = get_cred_profiles().await;
  let mut all_configs = HashMap::new();
  for (deploy_profile_name, deploy_profiles) in anycloud_prof.into_iter() {
    let mut configs = Vec::new();
    for profile in deploy_profiles {
      let cred_prof_name = match profile.credentialProfile {
        None => {
          if cred_profs.len() != 1 {
            let err = format!(
              "No credential profile specified in deploy config {} when more than one \
              credential profile exists in {}.",
              deploy_profile_name, CREDENTIALS_FILE
            );
            error!("INVALID_DEFAULT_CREDENTIAL_ALIAS", "{}", err).await;
            std::process::exit(1);
          }
          cred_profs.keys().next().unwrap().to_string()
        }
        Some(key) => key,
      };
      match cred_profs.get(&cred_prof_name) {
        Some(credentials) => {
          configs.push(Config {
            credentials: credentials.credentials.clone(),
            cloudProvider: credentials.cloudProvider.to_string(),
            region: profile.region,
            vmType: profile.vmType,
          });
        }
        None => {
          let err = format!(
            "Credentials {} for deploy config {} not found in {}",
            cred_prof_name, deploy_profile_name, CREDENTIALS_FILE
          );
          error!("INVALID_CREDENTIAL_ALIAS", "{}", err).await;
          std::process::exit(1);
        }
      }
    }
    all_configs.insert(deploy_profile_name, configs);
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
    Err(e) => return Err(PostV1Error::Other(e.to_string())),
  };
  let resp = CLIENT.request(req).await;
  let mut resp = match resp {
    Ok(resp) => resp,
    Err(e) => return Err(PostV1Error::Other(e.to_string())),
  };
  let data = hyper::body::to_bytes(resp.body_mut()).await;
  let data = match data {
    Ok(data) => data,
    Err(e) => return Err(PostV1Error::Other(e.to_string())),
  };
  let data_str = String::from_utf8(data.to_vec());
  let data_str = match data_str {
    Ok(data_str) => data_str,
    Err(e) => return Err(PostV1Error::Other(e.to_string())),
  };
  return match resp.status() {
    st if st.is_success() => Ok(data_str),
    StatusCode::REQUEST_TIMEOUT => Err(PostV1Error::Timeout),
    StatusCode::FORBIDDEN => Err(PostV1Error::Forbidden),
    StatusCode::CONFLICT => Err(PostV1Error::Conflict),
    _ => Err(PostV1Error::Other(data_str.to_string())),
  };
}

pub async fn client_error(err_name: &str, message: &str) {
  let mut body = json!({
    "errorName": err_name,
    "accessToken": get_token(),
    "alanVersion": format!("v{}", ALAN_VERSION),
    "osName": std::env::consts::OS,
    "message": message,
  });
  if let Some(cluster_id) = CLUSTER_ID.get() {
    body
      .as_object_mut()
      .unwrap()
      .insert(format!("clusterId"), json!(cluster_id));
  }
  let _resp = post_v1("clientError", body).await;
}

pub async fn terminate(cluster_id: &str) {
  let body = json!({
    "deployConfig": get_config().await,
    "clusterId": cluster_id,
    "accessToken": get_token(),
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
      PostV1Error::Unauthorized => {
        clear_token();
        format!("{}", UNAUTHORIZED_OPERATION)
      }
      PostV1Error::Other(err) => format!("Failed to terminate app {}. Error: {}", cluster_id, err),
    },
  };
  sp.message(res);
  sp.close();
}

pub async fn new(body: Value) {
  error!("TEST", "test error").await;
  let sp = SpinnerBuilder::new(format!("Creating new app")).start();
  let resp = post_v1("new", body).await;
  let res = match resp {
    Ok(res) => format!("Created app with id {} successfully!", res),
    Err(err) => match err {
      PostV1Error::Timeout => format!("{}", REQUEST_TIMEOUT),
      PostV1Error::Forbidden => format!("{}", FORBIDDEN_OPERATION),
      PostV1Error::Conflict => format!("Failed to create a new app. Error: {}", NAME_CONFLICT),
      PostV1Error::Unauthorized => {
        clear_token();
        format!("{}", UNAUTHORIZED_OPERATION)
      }
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
      PostV1Error::Unauthorized => {
        clear_token();
        format!("{}", UNAUTHORIZED_OPERATION)
      }
      PostV1Error::Other(err) => format!("Failed to create a new app. Error: {}", err),
    },
  };
  sp.message(res);
  sp.close();
}

pub async fn info() {
  let body = json!({
    "deployConfig": get_config().await,
    "accessToken": get_token(),
  });
  let response = post_v1("info", body).await;
  let resp = match &response {
    Ok(resp) => resp,
    Err(err) => {
      match err {
        PostV1Error::Timeout => {
          eprintln!("{}", REQUEST_TIMEOUT);
        }
        PostV1Error::Forbidden => {
          eprintln!("{}", FORBIDDEN_OPERATION);
        }
        PostV1Error::Conflict => {
          eprintln!(
            "Displaying status for apps failed with error: {}",
            NAME_CONFLICT
          );
        }
        PostV1Error::Unauthorized => {
          clear_token();
          eprintln!("{}", UNAUTHORIZED_OPERATION);
        }
        PostV1Error::Other(err) => {
          eprintln!("Displaying status for apps failed with error: {}", err);
        }
      }
      std::process::exit(1);
    }
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
    header: "Deploy Profile".into(),
    ..Column::default()
  };
  clusters.columns.insert(2, column);

  let column = Column {
    header: "Size".into(),
    ..Column::default()
  };
  clusters.columns.insert(3, column);

  let column = Column {
    header: "Status".into(),
    ..Column::default()
  };
  clusters.columns.insert(4, column);

  let mut app_data: Vec<Vec<&dyn Display>> = vec![];
  let mut profile_data: Vec<Vec<&dyn Display>> = vec![];
  let mut deploy_profiles = HashSet::new();
  for app in &mut apps {
    app_data.push(vec![
      &app.id,
      &app.url,
      &app.deployName,
      &app.size,
      &app.status,
    ]);
    if deploy_profiles.contains(&app.deployName) {
      continue;
    }
    for (i, profile) in app.cloudConfigs.iter().enumerate() {
      if i == 0 {
        profile_data.push(vec![&app.deployName, &profile.region, &profile.vmType])
      } else {
        profile_data.push(vec![&"", &profile.region, &profile.vmType])
      };
    }
    deploy_profiles.insert(&app.deployName);
  }

  println!("Status of all apps deployed:\n");
  clusters.print(app_data);

  let mut profiles = AsciiTable::default();
  profiles.max_width = 140;

  let column = Column {
    header: "Deploy Profile".into(),
    ..Column::default()
  };
  profiles.columns.insert(0, column);

  let column = Column {
    header: "Credential Profile".into(),
    ..Column::default()
  };
  profiles.columns.insert(1, column);

  let column = Column {
    header: "Cloud Provider".into(),
    ..Column::default()
  };
  profiles.columns.insert(2, column);

  let column = Column {
    header: "Region".into(),
    ..Column::default()
  };
  profiles.columns.insert(3, column);

  let column = Column {
    header: "VM Type".into(),
    ..Column::default()
  };
  profiles.columns.insert(4, column);
  println!("\nDeployment configurations used:\n");
  profiles.print(profile_data);
}
