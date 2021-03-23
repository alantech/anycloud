# anycloud.json

Anycloud deployments are configured using an `anycloud.json` file located within the top level folder of your project or repository next to your [`Dockerfile`](dockerfile.md). Each deployment will be keyed by a name and contain an array of cloud provider configurations with `region`, `vmType` and `credentials`. Possible values for  will have a different possible values depending on which cloud provider it is.

The schema for the `~/.anycloud/deploy.json` is as follows:

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

Each cloud provider configuration within the deployment array has a different configuration schema. It is possible to have multi-cloud and/or multi-region deployments by having multiple cloud provider configurations per deployment. `~/.anycloud/deploy.json` could look something like:

```javascript
{
  "multi-region-aws": [
    {
      "cloudProvider": "AWS",
      "region": "us-west-1",
      "vmType": "t2.micro",
      "credentials": {
        ...
      },
    },
    {
      "cloudProvider": "AWS",
      "region": "us-west-2",
      "vmType": "t2.micro",
      "credentials": {
        ...
      },
    }
  ]
}
```

## AWS

List of regions.

List of virtual machines types.

## GCP

List of regions.

List of virtual machines types.

## Azure

List of regions.

List of virtual machines types.

