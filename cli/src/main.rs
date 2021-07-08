use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

use clap::{crate_name, crate_version, App, AppSettings, SubCommand};

fn git_status() {
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
}

fn git_archive_app_tar(file_path: &str) {
  let output = Command::new("git")
    .arg("archive")
    .arg("--format=tar.gz")
    .arg("-o")
    .arg(file_path)
    .arg("HEAD")
    .output()
    .unwrap();
  if output.status.code().unwrap() != 0 {
    eprintln!("Your code must be managed by git in order to deploy correctly, please run `git init && git commit -am \"Initial commit\"` and try again.");
    std::process::exit(output.status.code().unwrap());
  }
}

pub fn make_app_tar_gz() {
  git_status();
  let file_name = "app.tar.gz";
  git_archive_app_tar(&file_name);
}

pub fn make_anycloud_agz() {
  let anycloud_agz = include_bytes!("../alan/anycloud.agz");
  let file = File::create("anycloud.agz");
  let mut file = match file {
    Ok(file) => file,
    Err(e) => {
      eprintln!("Error creating anycloud file. {}", e);
      std::process::exit(1);
    },
  };
  match file.write_all(anycloud_agz) {
    Ok(_) => {},
    Err(e) => {
      eprintln!("Error writing anycloud file. {}", e);
      std::process::exit(1);
    },
  };
}

pub fn main() {
  let desc: &str = &format!("{}", env!("CARGO_PKG_DESCRIPTION"));
  let app = App::new(crate_name!())
    .version(crate_version!())
    .about(desc)
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .subcommand(SubCommand::with_name("new")
      .about("Deploys your repository to a new App with a Deploy Config from anycloud.json")
      .arg_from_usage("-e, --env-file=[ENV_FILE] 'Specifies an optional environment file'")
      .arg_from_usage("[NON_INTERACTIVE] -n, --non-interactive 'Enables non-interactive CLI mode useful for scripting.'")
      .arg_from_usage("-a, --app-name=[APP_NAME] 'Specifies an optional app name.'")
      .arg_from_usage("-c, --config-name=[CONFIG_NAME] 'Specifies a config name, required only in non-interactive mode.'")
    )
    .subcommand(SubCommand::with_name("list")
      .about("Displays all the Apps deployed with the Deploy Configs from anycloud.json")
    )
    .subcommand(SubCommand::with_name("terminate")
      .about("Terminate an App hosted in one of the Deploy Configs from anycloud.json")
      .arg_from_usage("[NON_INTERACTIVE] -n, --non-interactive 'Enables non-interactive CLI mode useful for scripting.'")
      .arg_from_usage("-a, --app-name=[APP_NAME] 'Specifies an optional app name.'")
      .arg_from_usage("-c, --config-name=[CONFIG_NAME] 'Specifies a config name, required only in non-interactive mode.'")
    )
    .subcommand(SubCommand::with_name("upgrade")
      .about("Deploys your repository to an existing App hosted in one of the Deploy Configs from anycloud.json")
      .arg_from_usage("-e, --env-file=[ENV_FILE] 'Specifies an optional environment file relative path'")
      .arg_from_usage("[NON_INTERACTIVE] -n, --non-interactive 'Enables non-interactive CLI mode useful for scripting.'")
      .arg_from_usage("-a, --app-name=[APP_NAME] 'Specifies an optional app name.'")
      .arg_from_usage("-c, --config-name=[CONFIG_NAME] 'Specifies a config name, required only in non-interactive mode.'")
    )
    .subcommand(SubCommand::with_name("config")
      .about("Manage Deploy Configs used by Apps from the anycloud.json in the current directory")
      .setting(AppSettings::SubcommandRequiredElseHelp)
      .subcommand(SubCommand::with_name("new")
        .about("Add a new Deploy Config to the anycloud.json in the current directory and creates the file if it doesn't exist.")
      )
      .subcommand(SubCommand::with_name("list")
        .about("List all the Deploy Configs from the anycloud.json in the current directory")
      )
      .subcommand(SubCommand::with_name("edit")
        .about("Edit an existing Deploy Config from the anycloud.json in the current directory")
      )
      .subcommand(SubCommand::with_name("remove")
        .about("Remove an existing Deploy Config from the anycloud.json in the current directory")
      )
    )
    .subcommand(SubCommand::with_name("credentials")
      .about("Manage all Credentials used by Deploy Configs from the credentials file at ~/.anycloud/credentials.json")
      .setting(AppSettings::SubcommandRequiredElseHelp)
      .subcommand(SubCommand::with_name("new")
        .about("Add a new Credentials")
      )
      .subcommand(SubCommand::with_name("list")
        .about("List all the available Credentials")
      )
      .subcommand(SubCommand::with_name("edit")
        .about("Edit an existing Credentials")
      )
      .subcommand(SubCommand::with_name("remove")
        .about("Remove an existing Credentials")
      )
    );

  let matches = app.get_matches();
  match matches.subcommand() {
    ("new", Some(matches)) => {
      let mut new_cmd = Command::new("alan");
      new_cmd.arg("deploy").arg("new");
      if matches.values_of("NON_INTERACTIVE").is_some() {
        new_cmd.arg("-n");
      }
      if let Some(app_name) = matches.value_of("app-name") {
        new_cmd.arg("-a").arg(app_name);
      }
      if let Some(config_name) = matches.value_of("config-name") {
        new_cmd.arg("-c").arg(config_name);
      }
      make_anycloud_agz();
      new_cmd.arg("anycloud.agz");
      make_app_tar_gz();
      new_cmd.arg("-f");
      let mut extra_files = "app.tar.gz".to_string();
      if let Some(env_file) = matches.value_of("env-file") {
        extra_files = format!("{},{}", extra_files, env_file);
      }
      new_cmd.arg(extra_files);
      new_cmd.status().unwrap();
    }
    ("terminate", Some(matches)) => {
      let mut new_cmd = Command::new("alan");
      new_cmd.arg("deploy").arg("terminate");
      if matches.values_of("NON_INTERACTIVE").is_some() {
        new_cmd.arg("-n");
      }
      if let Some(app_name) = matches.value_of("app-name") {
        new_cmd.arg("-a").arg(app_name);
      }
      if let Some(config_name) = matches.value_of("config-name") {
        new_cmd.arg("-c").arg(config_name);
      }
      new_cmd.status().unwrap();
    }
    ("upgrade", Some(matches)) => {
      let mut new_cmd = Command::new("alan");
      new_cmd.arg("deploy").arg("upgrade");
      if matches.values_of("NON_INTERACTIVE").is_some() {
        new_cmd.arg("-n");
      }
      if let Some(app_name) = matches.value_of("app-name") {
        new_cmd.arg("-a").arg(app_name);
      }
      if let Some(config_name) = matches.value_of("config-name") {
        new_cmd.arg("-c").arg(config_name);
      }
      make_anycloud_agz();
      new_cmd.arg("anycloud.agz");
      make_app_tar_gz();
      new_cmd.arg("-f");
      let mut extra_files = "app.tar.gz".to_string();
      if let Some(env_file) = matches.value_of("env-file") {
        extra_files = format!("{},{}", extra_files, env_file);
      }
      new_cmd.arg(extra_files);
      new_cmd.status().unwrap();
    }
    ("list", _) => {
      Command::new("alan").arg("deploy").arg("list").status().unwrap();
    }
    ("credentials", Some(sub_matches)) => {
      match sub_matches.subcommand() {
        ("new", _) => {
          Command::new("alan").arg("deploy").arg("credentials").arg("new").status().unwrap();
        }
        ("edit", _) => {
          Command::new("alan").arg("deploy").arg("credentials").arg("edit").status().unwrap();
        },
        ("list", _) => {
          Command::new("alan").arg("deploy").arg("credentials").arg("list").status().unwrap();
        },
        ("remove", _) => {
          Command::new("alan").arg("deploy").arg("credentials").arg("remove").status().unwrap();
        },
        // rely on AppSettings::SubcommandRequiredElseHelp
        _ => {}
      }
    }
    ("config", Some(sub_matches)) => {
      match sub_matches.subcommand() {
        ("new", _) => {
          Command::new("alan").arg("deploy").arg("config").arg("new").status().unwrap();
        },
        ("edit", _) => {
          Command::new("alan").arg("deploy").arg("config").arg("edit").status().unwrap();
        },
        ("list", _) => {
          Command::new("alan").arg("deploy").arg("config").arg("list").status().unwrap();
        },
        ("remove", _) => {
          Command::new("alan").arg("deploy").arg("config").arg("remove").status().unwrap();
        },
        // rely on AppSettings::SubcommandRequiredElseHelp
        _ => {}
      }
    }
    // rely on AppSettings::SubcommandRequiredElseHelp
    _ => {}
  }
}
