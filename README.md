<div align="center">
  <img src="./assets/anycloud.png" alt="drawing" width="180"/>
  <h1>AnyCloud scales webservers in any cloud provider</h2>
</div>
<br/>

AnyCloud takes API credentials from your preferred cloud provider and a `Dockerfile` in order to elastically scale an HTTP server in your own cloud provider account. Our aim is providing a much better experience to alternative services offered in AWS (Lambda, Elastic Beanstalk, ECS/Fargate, EC2) or GCP (Cloud Functions, App Engine, Cloud Run, Compute Engine).

- [x] Automatically scales your HTTP server based on request load and system stats
- [x] Vendor portability across cloud providers
- [x] No need to provision and manage virtual machines
- [x] Runs in your local dev environment as-is
- [x] HTTPS support included out of the box
- [x] Supports multi-region and multi-cloud deployments
- [x] In-memory distributed datastore
- [ ] Web socket support
- [ ] Cron job support


## Project Status

AnyCloud has been restructured into a pure [Alan application](./alan/anycloud.ln) with the CLI simply being a convenience tool to wrap up your own project the way the `anycloud.ln` application expects it and provide the right arguments to the `alan deploy` command for you.

This means there's "nothing special" about AnyCloud now versus Alan from a technical perspective, but new feature development will be gated on improvements to the language to support those features instead of being developed in parallel.

## Supported Cloud Providers

AnyCloud is hosted directly in your own account with any of the following cloud providers:

- [x] AWS
- [x] GCP
- [x] Azure

We are not adding any more cloud providers at this time.

## Documentation

To get started visit our [docs](https://docs.anycloudapp.com)
