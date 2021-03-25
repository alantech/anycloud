# Project Structure

## .git

The AnyCloud CLI expects your project to be version controlled with `git`. However, it is not required for your repository to be hosted in a remote `git` server like GitHub or GitLab.

## Dockerfile

Deploying an AnyCloud app requires a [`Dockerfile`](https://docs.docker.com/engine/reference/builder/) file located within the top level folder of your project next to your `anycloud.json`. AnyCloud will expect the docker container described by the `Dockerfile` to have a HTTP server listening on port `8088` like the one [here](../tutorial.md#configure-your-project).

## anycloud.json

AnyCloud deployments are configured using an `anycloud.json` file located within the top level folder of your project or repository, next to your [`Dockerfile`](). Each deployment will be keyed by a name and contain an array of deployment configurations that are described by the following fields: `region`, `vmType` and `credentials`. The string in the`credentials` field has to match one of the keys from your JSON  [credentials](credentials.md) file defined in `~/.anycloud/credentials.json`. The schema for `anycloud.json` is as follows:

```javascript
{
  "string": [
    {
      "region": "string",
      "vmType": "string",
      "credentials": "string"
    }
  ],
  "string": [
    {
      "region": "string",
      "vmType": "string",
      "credentials": "string"
    },
    {
      "region": "string",
      "vmType": "string",
      "credentials": "string"
    }
  ]
  ...
}
```

An example of an `anycloud.json` with a staging and a production deployment configuration will look like this:

```javascript
{
  "staging": [{
    "cloudProvider": "AWS",
    "region": "us-west-1",
    "vmType": "t2.medium",
    "credentials": "piedpiper-aws"
  }],
  "production": [{
    "cloudProvider": "AWS",
    "region": "us-west-1",
    "vmType": "t3.xlarge",
    "credentials": "piedpiper-aws"
  }]
}
```



Each cloud provider will have a different possible values for `region` and `vmType`.

* **AWS**: List of available [regions](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-regions-availability-zones.html#concepts-available-regions) and [virtual machines](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html#AvailableInstanceTypes) types
* **GCP**: List of available [regions](https://cloud.google.com/compute/docs/regions-zones#available) and [virtual machines](https://cloud.google.com/compute/docs/machine-types) types
* **Azure**: List of available [regions](https://azure.microsoft.com/en-us/global-infrastructure/geographies/#geographies) and [virtual machines](https://docs.microsoft.com/en-us/azure/virtual-machines/sizes) types

Note: AnyCloud does not currently support any ARM based VMs.