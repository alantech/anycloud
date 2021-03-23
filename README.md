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
- [ ] In-memory distributed datastore
- [ ] Web socket support
- [ ] Cron job support


## Project Status

- [x] Alpha: We are working with closed set of customers. Drop us a line at hello at anycloudapp dot com if you are interested
- [ ] Beta: Anyone can sign up. Stable enough for most use-cases
- [ ] Public: Production-ready for enterprise use-cases

## Supported Cloud Providers

AnyCloud is hosted directly in your own account with any of the following cloud providers:

- [x] AWS
- [x] GCP
- [x] Azure
- [ ] Digital Ocean

## Documentation

To get started visit our [docs](https://docs.anycloudapp.com)
