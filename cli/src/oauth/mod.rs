use hyper::Request;
use serde_json::{json, Value};
use std::fs::{read_to_string, remove_file, File};
use std::io::prelude::*;
use std::path::Path;
use tokio::time::{sleep, Duration};

use log::error;

use crate::http::CLIENT;

const CODE_URL: &'static str = "https://github.com/login/device/code";
const CLIENT_ID: &'static str = "f6e1ede88556627925d6";
const GRANT_TYPE: &'static str = "urn:ietf:params:oauth:grant-type:device_code";
const CODE_BODY: &'static str = "{\
  \"client_id\": \"f6e1ede88556627925d6\",\
  \"scope\": \"user:email\"\
}";
const POLL_URL: &'static str = "https://github.com/login/oauth/access_token";
const ERR: &'static str = "Failed to perform OAuth 2.0 authentication with GitHub";
const TOKEN_FILE: &str = ".anycloud/.token";

// Get previously generated OAuth access token or generate a new one
pub async fn get_token() -> String {
  let home = std::env::var("HOME").unwrap();
  let file_name = &format!("{}/{}", home, TOKEN_FILE);
  match read_to_string(file_name) {
    Ok(token) => token,
    Err(_) => generate_token().await,
  }
}

pub fn clear_token() {
  let home = std::env::var("HOME").unwrap();
  let file_name = &format!("{}/{}", home, TOKEN_FILE);
  remove_file(file_name).unwrap();
}

// Prompts the user to authenticate with Github using the Device Flow.
// Generates the OAuth access token, stores it in a file for later use and returns it.
// https://docs.github.com/en/developers/apps/authorizing-oauth-apps#device-flow
async fn generate_token() -> String {
  let req = Request::post(CODE_URL)
    .header("Content-Type", "application/json")
    .header("Accept", "application/json")
    .body(CODE_BODY.into())
    .unwrap();
  let resp = CLIENT.request(req).await.expect(ERR);
  let data = hyper::body::to_bytes(resp.into_body()).await.expect(ERR);
  let data_str = String::from_utf8(data.to_vec()).expect(ERR);
  let json: Value = serde_json::from_str(&data_str).expect(ERR);
  let device_code = json["device_code"].as_str().unwrap();
  let verification_uri = json["verification_uri"].as_str().unwrap();
  let user_code = json["user_code"].as_str().unwrap();
  println!(
    "Login to AnyCloud via GitHub by copying this one-time code\n
    {}\n\nand pasting it in the following url:\n
    {}",
    user_code, verification_uri
  );
  let interval = json["interval"].as_u64().unwrap();
  let period = Duration::from_secs(interval + 1);
  let body = json!({
    "client_id": CLIENT_ID,
    "grant_type": GRANT_TYPE,
    "device_code": device_code,
  });
  loop {
    let req = Request::post(POLL_URL)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json")
      .body(body.to_string().into())
      .unwrap();
    let resp = CLIENT.request(req).await.expect(ERR);
    let data = hyper::body::to_bytes(resp.into_body()).await.expect(ERR);
    let data_str = String::from_utf8(data.to_vec()).expect(ERR);
    let json: Value = serde_json::from_str(&data_str).expect(ERR);
    if let Some(token) = json["access_token"].as_str() {
      let home = std::env::var("HOME").unwrap();
      let file_name = &format!("{}/{}", home, TOKEN_FILE);
      let path = Path::new(file_name);
      // remove old token, if it exists
      if path.exists() {
        remove_file(path).expect(ERR);
      }
      let mut file = File::create(file_name).expect(ERR);
      file.write_all(token.as_bytes()).expect(ERR);
      return token.to_string();
    } else if let Some(error) = json["error"].as_str() {
      if error != "authorization_pending" {
        eprintln!("Authentication failed. Please try again. Err: {}", error);
        error!("Authentication failed. Err: {}", error);
        std::process::exit(1);
      }
    }
    sleep(period).await;
  }
}