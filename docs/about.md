# AnyCloud Documentation

[AnyCloud](https://github.com/alantech/anycloud/blob/main/README.md) takes API credentials from your preferred cloud provider and a `Dockerfile` in order to elastically scale an HTTP server in your own cloud provider account.

## About the documentation

A high-level overview of how the AnyCloud documentation is organized will help you know how to quickly find what you are looking for:

* The [Tutorial](tutorial.md) will guide you from 0 to a sample Node.js HTTP server deployed in your AWS account with AnyCloud. Start here if you’re new to AnyCloud.
* [How-to guides](how-to-guides/) are recipes. They guide you through the steps involved in addressing key problems and use-cases. They are more advanced than the tutorial and assume some knowledge of how AnyCloud works.
* [Technical reference](reference/) for built-in APIs and JSON file configuration schemas. They describe how it works and how to use it but assume some knowledge of how AnyCloud works
* [Background Information](background-information/) discusses key topics and concepts at a fairly high level and provide useful explanations about the internals

  of how AnyCloud works.

## CLI Installation

#### MacOS

```bash
brew tap alantech/homebrew-core
brew install anycloud
```

For Linux and Windows it is recommended to install AnyCloud via the [published artifacts](https://github.com/alantech/anycloud/releases/latest). Simply download the zip or tar.gz file for your operating system, and extract the `anycloud` executable to somewhere in your `$PATH`, make sure it's marked executable \(if not on Windows\), and you're ready to roll.

#### Linux

```bash
wget https://github.com/alantech/anycloud/releases/latest/download/anycloud-ubuntu.tar.gz
tar -xzf anycloud-ubuntu.tar.gz
sudo mv anycloud /usr/local/bin/anycloud
```

#### Windows

```text
Invoke-WebRequest -OutFile anycloud-windows.zip -Uri https://github.com/alantech/anycloud/releases/latest/download/anycloud-windows.zip
Expand-Archive -Path anycloud-windows.zip -DestinationPath C:\windows
```
