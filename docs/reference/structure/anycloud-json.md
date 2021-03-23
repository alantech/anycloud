# anycloud.json

Anycloud deployments are configured using an `anycloud.json` file located within the top level folder of your project or repository, next to your [`Dockerfile`](dockerfile.md). Each deployment will be keyed by a name and contain an array of deployment configurations that are described by the following fields: `region`, `vmType` and `credentials`. The string in the`credentials` field has to match one of the keys from your JSON  [credentials](../credentials.md) file defined in `~/.anycloud/credentials.json`. The schema for `anycloud.json` is as follows:

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

An example of an `anycloud.json` with a staging and a production deployment configuration:

```javascript
{
  "staging": [{
    "cloudProvider": "AWS",
    "region": "us-west-1",
    "vmType": "t2.medium",
    "credentials": "stripe-aws",
  }],
  "production": [{
    "cloudProvider": "AWS",
    "region": "us-west-1",
    "vmType": "t3.xlarge",
    "credentials": "stripe-aws",
  }]
}
```



Each cloud provider will have a different possible values for `region` and `vmType`

## AWS

List of regions.

List of virtual machines types.

## GCP

List of regions.

List of virtual machines types.

## Azure

List of regions.

List of virtual machines types.
