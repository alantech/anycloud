use std::env;
use std::fs::read;
use std::process::Command;

use base64;
use clap::{crate_name, crate_version, App, AppSettings, SubCommand};
use serde_json::json;

use anycloud::deploy::{client_error, get_config, info, new, terminate, upgrade, ALAN_VERSION};
use anycloud::error;
use anycloud::oauth::{authenticate, get_token};

async fn get_dockerfile_b64() -> String {
  let pwd = env::current_dir();
  match pwd {
    Ok(pwd) => {
      let dockerfile = read(format!("{}/Dockerfile", pwd.display()))
        .expect(&format!("No Dockerfile in {}", pwd.display()));
      return base64::encode(dockerfile);
    }
    Err(_) => {
      error!("INVALID_PWD", "Current working directory value is invalid").await;
      std::process::exit(1);
    }
  }
}

async fn get_env_file_b64(env_file_path: String) -> String {
  let pwd = env::current_dir();
  match pwd {
    Ok(pwd) => {
      let env_file = read(format!("{}/{}", pwd.display(), env_file_path));
      match env_file {
        Ok(env_file) => base64::encode(env_file),
        Err(_) => {
          error!("NO_ENV_FILE", "No environment file in {}/{}", pwd.display(), env_file_path).await;
          std::process::exit(1);
        }
      }
    }
    Err(_) => {
      error!("INVALID_PWD", "Current working directory value is invalid").await;
      std::process::exit(1);
    }
  }
}

async fn get_app_tar_gz_b64() -> String {
  let output = Command::new("git")
    .arg("status")
    .arg("--porcelain")
    .output()
    .unwrap();

  let msg = String::from_utf8(output.stdout).unwrap();
  if msg.contains("M ") {
    error!(
      "GIT_CHANGES",
      "Please stash, commit or .gitignore your changes before deploying and try again:\n\n{}",
      msg
    ).await;
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
    error!("NO_GIT", "Your code must be managed by git in order to deploy correctly, please run `git init && git commit -am \"Initial commit\"` and try again.").await;
    std::process::exit(output.status.code().unwrap());
  }

  let pwd = std::env::var("PWD").unwrap();
  let app_tar_gz = read(format!("{}/app.tar.gz", pwd)).expect("app.tar.gz was not generated");

  let output = Command::new("rm").arg("app.tar.gz").output().unwrap();

  if output.status.code().unwrap() != 0 {
    error!("DELETE_TMP_TAR", "Somehow could not delete temporary app.tar.gz file").await;
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
      .about("Deploys your repository to a new app with a deploy profile from anycloud.json")
      .arg_from_usage("-p --deploy-profile=[DEPLOY_PROFILE] 'Specifies the name of a deploy profile from anycloud.json. Required if there are multiple profiles'")
      .arg_from_usage("-a, --app-id=[APP_ID] 'Specifies an optional application identifier'")
      .arg_from_usage("-e, --env-file=[ENV_FILE] 'Specifies an optional environment file'")
    )
    .subcommand(SubCommand::with_name("info")
      .about("Displays all the apps deployed with the deploy profiles from anycloud.json")
    )
    .subcommand(SubCommand::with_name("terminate")
      .about("Terminate an app with the provided ID hosted in one of the deploy profiles from anycloud.json")
      .arg_from_usage("<APP_ID> 'Specifies the alan app to terminate'")
    )
    .subcommand(SubCommand::with_name("upgrade")
      .about("Deploys your repository to an existing app hosted in one of the deploy profiles from anycloud.json")
      .arg_from_usage("<APP_ID> 'Specifies the alan app to upgrade'")
      .arg_from_usage("-e, --env-file=[ENV_FILE] 'Specifies an optional environment file relative path'")
    );

  authenticate().await;
  let matches = app.get_matches();
  match matches.subcommand() {
    ("new", Some(matches)) => {
      let config = get_config().await;
      let profile = match matches.value_of("deploy-profile") {
        None => {
          if config.len() != 1 {
            let err = format!(
              "No deploy profile from anycloud.json specified when more than one \
              profile exists.",
            );
            error!("INVALID_DEFAULT_ANYCLOUD_ALIAS", "{}", err).await;
            std::process::exit(1);
          }
          config.keys().next().unwrap().to_string()
        }
        Some(key) => key.to_string(),
      };
      if !config.contains_key(&profile) {
        error!("DEPLOY_NOT_FOUND", "Deploy name provided is not defined in anycloud.json").await;
        std::process::exit(1);
      }
      let app_id = matches.value_of("app-id");
      let env_file = matches.value_of("env-file");
      let mut body = json!({
        "deployConfig": config,
        "deployName": profile,
        "agzB64": anycloud_agz,
        "DockerfileB64": get_dockerfile_b64().await,
        "appTarGzB64": get_app_tar_gz_b64().await,
        "appId": app_id,
        "alanVersion": format!("v{}", ALAN_VERSION),
        "osName": std::env::consts::OS,
        "accessToken": get_token(),
      });
      if let Some(env_file) = env_file {
        body.as_object_mut().unwrap().insert(
          format!("envB64"),
          json!(get_env_file_b64(env_file.to_string()).await),
        );
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
      let mut body = json!({
        "clusterId": cluster_id,
        "deployConfig": config,
        "agzB64": anycloud_agz,
        "DockerfileB64": get_dockerfile_b64().await,
        "appTarGzB64": get_app_tar_gz_b64().await,
        "alanVersion": format!("v{}", ALAN_VERSION),
        "accessToken": get_token(),
        "osName": std::env::consts::OS,
      });
      if let Some(env_file) = env_file {
        body.as_object_mut().unwrap().insert(
          format!("envB64"),
          json!(get_env_file_b64(env_file.to_string()).await),
        );
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
