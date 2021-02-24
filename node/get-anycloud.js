#!/usr/bin/env node

const { exec, } = require('child_process');
const path = require('path');

const pjson = require('./package.json');

const anycloudUrlBase = `https://github.com/alantech/anycloud/releases/download/v${pjson.version}/`;
let request = '';
let extract = '';
let file = 'anycloud-ubuntu.tar.gz';
if (process.platform === 'darwin') {
  file = 'anycloud-macos.tar.gz';
  request = `curl -OL ${anycloudUrlBase}${file}`;
  extract = `tar -xzf ${file}`;
} else if (process.platform === 'win32') {
  file = 'anycloud-windows.zip';
  request = `Invoke-WebRequest -OutFile anycloud-windows.zip -Uri ${anycloudUrlBase}${file}`;
  extract = 'Expand-Archive -Path anycloud-windows.zip -DestinationPath .';
} else {
  request = `curl -OL ${anycloudUrlBase}${file}`;
  extract = `tar -xzf ${file}`;
}

exec('mkdir bin', (error, stdout, stderr) => {
  if (error) {
    console.log(stdout);
    console.error(stderr);
    process.exit(1);
  }
  const cwd = path.join(process.cwd(), 'bin')
  exec(request, { cwd, }, (error, stdout, stderr) => {
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