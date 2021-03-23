# Tutorial

In this tutorial we will deploy the [sample express Node.js HTTP server](https://expressjs.com/en/starter/hello-world.html) in your own AWS account with AnyCloud. All the code can be found here.

## Enable programatic AWS access for AnyCloud

1. Create a new an IAM user in your AWS account in their UI as described [here](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_users_create.html#id_users_create_console).
2. Create a new access key under that IAM user in their UI as described [here](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_access-keys.html#Using_CreateAccessKey).
3. Enable programmatic access for that IAM user, and attach the built-in [`AmazonEC2FullAccess`](https://console.aws.amazon.com/iam/home#/policies/arn%3Aaws%3Aiam%3A%3Aaws%3Apolicy%2FAmazonEC2FullAccess)policy to it as described [here](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_manage-attach-detach.html#add-policies-console).
4. Add a local `~/.anycloud/credentials.json` file

## Configure your project

1. Initialize a `git` repository

```bash
git init
git add -A
git commit -m "Initial commit"
```

2. Initialize your `package.json` and install `express`

```bash
npm init
npm install express --save
```

3. Define the entry point into an HTTP server listening on port `8088` in an `index.js` file

```javascript
const express = require('express')
const app = express()
const port = process.env.PORT

app.get('/', (req, res) => {
  res.send('Hello World!')
})

app.listen(port, () => {
  console.log(`Example app listening at http://localhost:${port}`)
})
```

4. Define the `Dockerfile`

```text
FROM node:lts

ENV PORT 8088

COPY . .

RUN npm install
CMD node index.js
```

5. Test the `Dockerfile` locally by installing [Docker Desktop](https://www.docker.com/products/docker-desktop), building the Docker image and then running the server within the container

```text
docker build -t anycloud/app .
docker run -p 8088:8088 -d anycloud/app:latest
curl localhost:8088
```

Which should return `Hello World!`

 6. Add a local `anycloud.json`

