#!/usr/bin/env node

const { exec, } = require('child_process');
const path = require('path');

const anycloudUrlBase = 'https://github.com/alantech/anycloud/releases/download/v0.1.1/';
let extract = '';
let file = 'anycloud-ubuntu.tar.gz';
if (process.platform === 'darwin') {
  file = 'anycloud-macos.tar.gz';
  extract = `tar -xzf ${file}`;
} else if (process.platform === 'win32') {
  file = 'anycloud-windows.zip';
  extract = 'Expand-Archive -Path anycloud-windows.zip -DestinationPath .';
} else {
  extract = `tar -xzf ${file}`;
}

exec('mkdir bin', (error, stdout, stderr) => {
  if (error) {
    console.log(stdout);
    console.error(stderr);
    process.exit(1);
  }
  const cwd = path.join(process.cwd(), 'bin')
  exec(`curl -OL ${anycloudUrlBase}${file}`, { cwd, }, (error, stdout, stderr) => {
    if (error) {
      console.log(stdout);
      console.error(stderr);
      process.exit(2);
    }
    exec(extract, { cwd, }, (error, stdout, stderr) => {
      if (error) {
        console.log(stdout);
        console.error(stderr);
        process.exit(3);
      }
    });
  });
});