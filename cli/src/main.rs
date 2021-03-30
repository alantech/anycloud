use std::env;
use std::fs::read;
use std::process::Command;

use base64;
use clap::{crate_name, crate_version, App, AppSettings, SubCommand};
use serde_json::json;

use anycloud::deploy::{client_error, get_config, info, new, terminate, upgrade, ALAN_VERSION};
use anycloud::oauth::{authenticate, get_token};

async fn get_dockerfile_b64(cluster_id: Option<&str>) -> String {
  let pwd = env::current_dir();
  match pwd {
    Ok(pwd) => {
      let dockerfile = read(format!("{}/Dockerfile", pwd.display()))
        .expect(&format!("No Dockerfile in {}", pwd.display()));
      return base64::encode(dockerfile);
    }
    Err(_) => {
      let err_str = format!("Current working directory value is invalid");
      eprintln!("{}", err_str);
      client_error("INVALID_PWD", Some(&err_str), cluster_id).await;
      std::process::exit(1);
    }
  }
}

async fn get_env_file_b64(env_file_path: String, cluster_id: Option<&str>) -> String {
  let pwd = env::current_dir();
  match pwd {
    Ok(pwd) => {
      let env_file = read(format!("{}/{}", pwd.display(), env_file_path));
      match env_file {
        Ok(env_file) => base64::encode(env_file),
        Err(_) => {
          let err_str = format!("No environment file in {}/{}", pwd.display(), env_file_path);
          eprintln!("{}", err_str);
          client_error("NO_ENV_FILE", Some(&err_str), cluster_id).await;
          std::process::exit(1);
        }
      }
    }
    Err(_) => {
      let err_str = format!("Current working directory value is invalid");
      eprintln!("{}", err_str);
      client_error("INVALID_PWD", Some(&err_str), cluster_id).await;
      std::process::exit(1);
    }
  }
}

async fn get_app_tar_gz_b64(cluster_id: Option<&str>) -> String {
  let output = Command::new("git")
    .arg("status")
    .arg("--porcelain")
    .output()
    .unwrap();

  let msg = String::from_utf8(output.stdout).unwrap();
  if msg.contains("M ") {
    let err_str = format!(
      "Please stash, commit or .gitignore your changes before deploying and try again:\n\n{}",
      msg
    );
    eprintln!("{}", err_str);
    client_error("GIT_CHANGES", Some(&err_str), cluster_id).await;
    std::process::exit(1);
  }

  let output = Command::new("git")
    .arg("archive")
    .arg("--format=tar.gz")
    .arg("-o")
    .arg("app.tar.gz")
    .arg("HEAD")
    .output()
    .unwrap();

  if output.status.code().unwrap() != 0 {
    let err_str = format!("Your code must be managed by git in order to deploy correctly, please run `git init && git commit -am \"Initial commit\"` and try again.");
    eprintln!("{}", err_str);
    client_error("NO_GIT", Some(&err_str), cluster_id).await;
    std::process::exit(output.status.code().unwrap());
  }

  let pwd = std::env::var("PWD").unwrap();
  let app_tar_gz = read(format!("{}/app.tar.gz", pwd)).expect("app.tar.gz was not generated");

  let output = Command::new("rm").arg("app.tar.gz").output().unwrap();

  if output.status.code().unwrap() != 0 {
    let err_str = format!("Somehow could not delete temporary app.tar.gz file");
    eprintln!("{}", err_str);
    client_error("DELETE_TMP_TAR", Some(&err_str), cluster_id).await;
    std::process::exit(output.status.code().unwrap());
  }

  return base64::encode(app_tar_gz);
}

#[tokio::main]
pub async fn main() {
  let anycloud_agz = base64::encode(include_bytes!("../alan/anycloud.agz"));
  let desc: &str = &format!("alan {}\n{}", ALAN_VERSION, env!("CARGO_PKG_DESCRIPTION"));
  let app = App::new(crate_name!())
    .version(crate_version!())
    .about(desc)
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .subcommand(SubCommand::with_name("new")
      .about("Deploys your repository to a new app with one of the deploy configs from anycloud.json")
      .arg_from_usage("<DEPLOY_NAME> 'Specifies the name of the deploy config to use'")
      .arg_from_usage("-a, --app-id=[APP_ID] 'Specifies an optional application identifier'")
      .arg_from_usage("-e, --env-file=[ENV_FILE] 'Specifies an optional environment file'")
    )
    .subcommand(SubCommand::with_name("info")
      .about("Displays all the apps deployed with the deploy config from anycloud.json")
    )
    .subcommand(SubCommand::with_name("terminate")
      .about("Terminate an app with the provided id hosted in one of the deploy configs at anycloud.json")
      .arg_from_usage("<APP_ID> 'Specifies the alan app to terminate'")
    )
    .subcommand(SubCommand::with_name("upgrade")
      .about("Deploys your repository to an existing app hosted in one of the deploy configs at anycloud.json")
      .arg_from_usage("<APP_ID> 'Specifies the alan app to upgrade'")
      .arg_from_usage("-e, --env-file=[ENV_FILE] 'Specifies an optional environment file relative path'")
    );

  authenticate().await;
  let matches = app.get_matches();
  match matches.subcommand() {
    ("new", Some(matches)) => {
      let config = get_config().await;
      let deploy_name = matches.value_of("DEPLOY_NAME").unwrap();
      if !config.contains_key(deploy_name) {
        let err_str = format!("Deploy name provided is not defined in anycloud.json");
        eprintln!("{}", err_str);
        client_error("NO_DEPLOY_MATCH", Some(&err_str), None).await;
        std::process::exit(1);
      }
      let app_id = matches.value_of("app-id");
      let env_file = matches.value_of("env-file");
      let dockerfile_b64 = get_dockerfile_b64(None).await;
      let app_tar_gz_b64 = get_app_tar_gz_b64(None).await;
      let mut body = json!({
        "deployConfig": config,
        "deployName": deploy_name,
        "agzB64": anycloud_agz,
        "DockerfileB64": dockerfile_b64,
        "appTarGzB64": app_tar_gz_b64,
        "appId": app_id,
        "alanVersion": format!("v{}", ALAN_VERSION),
        "osName": std::env::consts::OS,
        "accessToken": get_token(),
      });
      if let Some(env_file) = env_file {
        let env_file_b64 = get_env_file_b64(env_file.to_string(), None).await;
        body
          .as_object_mut()
          .unwrap()
          .insert(format!("envB64"), json!(env_file_b64));
      }
      new(body).await;
    }
    ("terminate", Some(matches)) => {
      let cluster_id = matches.value_of("APP_ID").unwrap();
      terminate(cluster_id).await;
    }
    ("upgrade", Some(matches)) => {
      let config = get_config().await;
      let cluster_id = matches.value_of("APP_ID").unwrap();
      let env_file = matches.value_of("env-file");
      let dockerfile_b64 = get_dockerfile_b64(Some(&cluster_id)).await;
      let app_tar_gz_b64 = get_app_tar_gz_b64(Some(&cluster_id)).await;
      let mut body = json!({
        "clusterId": cluster_id,
        "deployConfig": config,
        "agzB64": anycloud_agz,
        "DockerfileB64": dockerfile_b64,
        "appTarGzB64": app_tar_gz_b64,
        "alanVersion": format!("v{}", ALAN_VERSION),
        "accessToken": get_token(),
        "osName": std::env::consts::OS,
      });
      if let Some(env_file) = env_file {
        let env_file_b64 = get_env_file_b64(env_file.to_string(), Some(&cluster_id)).await;
        body
          .as_object_mut()
          .unwrap()
          .insert(format!("envB64"), json!(env_file_b64));
      }
      upgrade(body).await;
    }
    ("info", _) => {
      info().await;
    }
    // rely on AppSettings::SubcommandRequiredElseHelp
    _ => {}
  }
}
