# CLI

## Installation

### MacOS

```bash
brew tap alantech/homebrew-core
brew install anycloud
```

For Linux and Windows it is recommended to install AnyCloud via the [published artifacts](https://github.com/alantech/anycloud/releases/latest). Simply download the zip or tar.gz file for your operating system, and extract the `anycloud` executable to somewhere in your `$PATH`, make sure it's marked executable \(if not on Windows\), and you're ready to roll.

### Linux

```bash
wget https://github.com/alantech/anycloud/releases/latest/download/anycloud-ubuntu.tar.gz
tar -xzf anycloud-ubuntu.tar.gz
sudo mv anycloud /usr/local/bin/anycloud
```

### Windows PowerShell

```bash
Invoke-WebRequest -OutFile anycloud-windows.zip -Uri https://github.com/alantech/anycloud/releases/latest/download/anycloud-windows.zip
Expand-Archive -Path anycloud-windows.zip -DestinationPath C:\windows
```

## Authentication

Authentication in the CLI occurs via GitHub. Copy the code 

```bash
Login to AnyCloud via GitHub by copying this one-time code

    705C-0032

and pasting it in the following url:

    https://github.com/login/device
```

Make sure you are logged into Github. Open the provided url and you will see the following screen.

<img src="assets/gh-code.png" width=700 height=600 />

Paste your code, authorize the AnyCloud app and return to the terminal!