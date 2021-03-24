# Architecture

AnyCloud is built on the [Rust](https://rust-lang.org) and [Alan](https://alan-lang.org) programming languages. It is accessed via a CLI that takes [cloud provider credentials](../reference/credentials.md), a [deploy configuration]() and a `Dockerfile` with an HTTP server listening on port 8088 as input. The server running in your container is deployed to the account with the specified credentials and runs with a sidecar process that manages your server across multiple regions and cloud providers via DNS and figures out when to scale up or down.

More coming soon!

