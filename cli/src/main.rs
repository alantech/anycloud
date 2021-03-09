use std::env;
use std::fs::read;
use std::process::Command;

use base64;
use clap::{crate_name, crate_version, App, AppSettings, SubCommand};
use serde_json::json;

use anycloud::deploy::{info, get_config, new, terminate, upgrade};

fn get_dockerfile_b64() -> String {
  let pwd = std::env::var("PWD").unwrap();
  let dockerfile = read(format!("{}/Dockerfile", pwd)).expect(&format!("No Dockerfile in {}", pwd));
  return base64::encode(dockerfile);
}

fn get_app_tar_gz_b64() -> String {
  let output = Command::new("git")
    .arg("status")
    .arg("--porcelain")
    .output()
    .unwrap();

  let msg = String::from_utf8(output.stdout).unwrap();
  if msg.contains("M ") {
    eprintln!("Please stash, commit or .gitignore your changes before deploying and try again:\n\n{}", msg);
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
    eprintln!("Your code must be managed by git in order to deploy correctly, please run `git init && git commit -am \"Initial commit\"` and try again.");
    std::process::exit(output.status.code().unwrap());
  }

  let pwd = std::env::var("PWD").unwrap();
  let app_tar_gz = read(format!("{}/app.tar.gz", pwd)).expect("app.tar.gz was not generated");

  let output = Command::new("rm")
    .arg("app.tar.gz")
    .output()
    .unwrap();

  if output.status.code().unwrap() != 0 {
    eprintln!("Somehow could not delete temporary app.tar.gz file");
    std::process::exit(output.status.code().unwrap());
  }

  return base64::encode(app_tar_gz);
}

#[tokio::main]
pub async fn main() {
  let anycloud_agz = base64::encode(include_bytes!("../alan/anycloud.agz"));
  let app = App::new(crate_name!())
    .version(crate_version!())
    .about("AnyCloud is a Lambda alternative that works with multiple cloud provider.")
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .subcommand(SubCommand::with_name("new")
      .about("Deploys your repository to a new app with one of the deploy configs from anycloud.json")
      .arg_from_usage("<DEPLOY_NAME> 'Specifies the name of the deploy config to use'")
      .arg_from_usage("-a, --app-id=[APP_ID] 'Specifies an optional application identifier'")
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
    );

  let matches = app.get_matches();
  match matches.subcommand() {
    ("new",  Some(matches)) => {
      let config = get_config();
      let deploy_name = matches.value_of("DEPLOY_NAME").unwrap();
      if !config.contains_key(deploy_name) {
        eprintln!("Deploy name provided is not defined in anycloud.json");
        std::process::exit(1);
      }
      let app_id = matches.value_of("app-id");
      let body = json!({
        "deployConfig": config,
        "deployName": deploy_name,
        "agzB64": anycloud_agz,
        "DockerfileB64": get_dockerfile_b64(),
        "appTarGzB64": get_app_tar_gz_b64(),
        "appId": app_id,
      });
      new(body).await;
    },
    ("terminate",  Some(matches)) => {
      let cluster_id = matches.value_of("APP_ID").unwrap();
      terminate(cluster_id).await;
    },
    ("upgrade",  Some(matches)) => {
      let config = get_config();
      let cluster_id = matches.value_of("APP_ID").unwrap();
      let body = json!({
        "clusterId": cluster_id,
        "deployConfig": config,
        "agzB64": anycloud_agz,
        "DockerfileB64": get_dockerfile_b64(),
        "appTarGzB64": get_app_tar_gz_b64(),
      });
      upgrade(body).await;
    },
    ("info",  _) => {
      info().await;
    },
    // rely on AppSettings::SubcommandRequiredElseHelp
    _ => {}
  }
}
