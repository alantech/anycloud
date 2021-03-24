# Dockerfile

Deploying an AnyCloud app requires a [`Dockerfile`](https://docs.docker.com/engine/reference/builder/) file located within the top level folder of your project next to your [`anycloud.json`](anycloud-json.md). AnyCloud will expect the docker container described by the `Dockerfile` to have a HTTP server listening on port `8088` like the one [here](../../tutorial.md#configure-your-project).

