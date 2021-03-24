# CLI API

## help

```
$ anycloud help
Elastically scale webservers in any cloud provider

USAGE:
    anycloud <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help         Prints this message or the help of the given subcommand(s)
    info         Displays all the apps deployed with the deploy config from anycloud.json
    new          Deploys your repository to a new app with one of the deploy configs from anycloud.json
    terminate    Terminate an app with the provided id hosted in one of the deploy configs at anycloud.json
    upgrade      Deploys your repository to an existing app hosted in one of the deploy configs at anycloud.json
```

## new

```
$ anycloud help new
Deploys your repository to a new app with one of the deploy configs from anycloud.json

USAGE:
    anycloud new [OPTIONS] <DEPLOY_NAME>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --app-id <APP_ID>    Specifies an optional application identifier
    -e, --env-file=<ENV_FILE> Specifies an optional environment file

ARGS:
    <DEPLOY_NAME>    Specifies the name of the deploy config to use
```

## upgrade

```
$ anycloud help upgrade
Deploys your repository to an existing app hosted in one of the deploy configs at anycloud.json

USAGE:
    anycloud upgrade <APP_ID>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --env-file=<ENV_FILE> Specifies an optional environment file

ARGS:
    <APP_ID>    Specifies the alan app to upgrade
```

## info

```
$ anycloud help info
Displays all the apps deployed with the deploy config from anycloud.json

USAGE:
    anycloud info

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
```

## terminate

```
$ anycloud help terminate
Terminate an app with the provided id hosted in one of the deploy configs at anycloud.json

USAGE:
    anycloud terminate <APP_ID>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <APP_ID>    Specifies the alan app to terminate
```