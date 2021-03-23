# Dockerfile

Anycloud deployments expect a `Dockerfile` file located within the top level folder of your project next to your [`anycloud.json`](anycloud-json.md). AnyCloud will expect the docker container described by the `Dockerfile` to have a HTTP server listening on port `8088`. If the server listens on the port passed in through an environment variable called `PORT`, the `Dockerfile` could look like this:

```text
FROM node:lts

ENV PORT 8088

COPY . .

RUN yarn
RUN yarn build
CMD yarn start
```

You can your `Dockerfile` will work locally by running:

```text
docker build -t anycloud/app .
```

and then:

```text
docker run -p 8088:8088 -d anycloud/app:latest
```

