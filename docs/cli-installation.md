# CLI Installation

## **MacOS**

```bash
brew tap alantech/homebrew-core
brew install anycloud
```

For Linux and Windows it is recommended to install AnyCloud via the [published artifacts](https://github.com/alantech/anycloud/releases/latest). Simply download the zip or tar.gz file for your operating system, and extract the `anycloud` executable to somewhere in your `$PATH`, make sure it's marked executable \(if not on Windows\), and you're ready to roll.

## **Linux**

```bash
wget https://github.com/alantech/anycloud/releases/latest/download/anycloud-ubuntu.tar.gz
tar -xzf anycloud-ubuntu.tar.gz
sudo mv anycloud /usr/local/bin/anycloud
```

## **Windows**

```text
Invoke-WebRequest -OutFile anycloud-windows.zip -Uri https://github.com/alantech/anycloud/releases/latest/download/anycloud-windows.zip
Expand-Archive -Path anycloud-windows.zip -DestinationPath C:\windows
```

