use std::env;

use clap::{crate_name, crate_version, App, AppSettings, SubCommand};

use anycloud::deploy::deploy::{info, new, terminate, upgrade};

#[tokio::main]
pub async fn main() {
  let app = App::new(crate_name!())
    .version(crate_version!())
    .about("AnyCloud is a Lambda alternative that works with multiple cloud provider.")
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .subcommand(SubCommand::with_name("new")
      .about("Deploys an .agz file to a new app in one of the cloud providers described in the deploy config at ~/.alan/deploy.json")
      .arg_from_usage("<AGZ_FILE> 'Specifies the .agz file to deploy'")
      .arg_from_usage("<CLOUD_ALIAS> 'Specifies the cloud provider to deploy to based on its alias'")
    )
    .subcommand(SubCommand::with_name("info")
      .about("Displays all the apps deployed in the cloud provider described in the deploy config at ~/.alan/deploy.json")
    )
    .subcommand(SubCommand::with_name("terminate")
      .about("Terminate an app with the provided id in the cloud provider described in the deploy config at ~/.alan/deploy.json")
      .arg_from_usage("<APP_ID> 'Specifies the alan app to terminate'")
    )
    .subcommand(SubCommand::with_name("upgrade")
      .about("Deploys an .agz file to an existing app in the cloud provider described in the deploy config at ~/.alan/deploy.json")
      .arg_from_usage("<APP_ID> 'Specifies the alan app to upgrade'")
      .arg_from_usage("<AGZ_FILE> 'Specifies the .agz file to deploy'")
    );

  let matches = app.get_matches();
  match matches.subcommand() {
    ("new",  Some(matches)) => {
      let agz_file = matches.value_of("AGZ_FILE").unwrap();
      let cloud_alias = matches.value_of("CLOUD_ALIAS").unwrap();
      new(agz_file, cloud_alias).await;
    },
    ("terminate",  Some(matches)) => {
      let app_id = matches.value_of("APP_ID").unwrap();
      terminate(app_id).await;
    },
    ("upgrade",  Some(matches)) => {
      let app_id = matches.value_of("APP_ID").unwrap();
      let agz_file = matches.value_of("AGZ_FILE").unwrap();
      upgrade(app_id, agz_file).await;
    },
    ("info",  _) => {
      info().await;
    },
    // rely on AppSettings::SubcommandRequiredElseHelp
    _ => {}
  }
}
