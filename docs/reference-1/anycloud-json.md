# anycloud.json

Anycloud deployments are configured using a local file in `~/.anycloud/credentials.json`. Each deployment will be keyed by a name and contain an array of cloud provider configurations with `credentials`, `region`, `vmType` and `cloudProvider`. Possible values for `cloudProvider` are [`AWS`](https://github.com/alantech/anycloud/tree/703791e7d87dabf056a1673f413a76e8c0ee2383/docs/start.md#aws), [`GCP`](https://github.com/alantech/anycloud/tree/703791e7d87dabf056a1673f413a76e8c0ee2383/docs/start.md#gcp) and [`Azure`](https://github.com/alantech/anycloud/tree/703791e7d87dabf056a1673f413a76e8c0ee2383/docs/start.md#azure) and the other fields will have a different possible values depending on which cloud provider it is.

The schema for the `~/.anycloud/deploy.json` is as follows:

```javascript
{
  "string": [
    {
      "cloudProvider": "string",
      "region": "string",
      "vmType": "string",
      "credentials" {
        ...
      }
    }
  ],
  "string": [
    {
      "cloudProvider": "string",
      "region": "string",
      "vmType": "string",
      "credentials" {
        ...
      }
    },
    {
      "cloudProvider": "string",
      "region": "string",
      "vmType": "string",
      "credentials" {
        ...
      }
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

The schema for the AWS deployment config is as follows:

```javascript
[
  ...
  {
    "cloudProvider": "string",
    "region": "string",
    "vmType": "string",
    "credentials": {
      "accessKeyId": "string",
      "secretAccessKey": "string",
    }
  }
  ...
]
```

Follow the steps at [credentials]() to get the `accessKeyId` and `secretAccessKey`. An example would look like:

```javascript
[
  ...
  {
    "cloudProvider": "AWS",
    "region": "us-west-1",
    "vmType": "t2.micro",
    "credentials": {
      "accessKeyId": "#####################",
      "secretAccessKey": "###################"
    },
  },
  ...
]
```

## GCP

The schema for a GCP deployment config is as follows:

```javascript
{
  "cloudProvider": "string",
  "region": "string",
  "vmType": "string",
  "credentials": {
    "privateKey": "string",
    "clientEmail": "string",
    "projectId": "string",
  }
}
```

Take a look at the exported JSON file from your GCP [credentials]() and grab your `project_id`, `private_key` and `client_email`. An example of a GCP cloud configuration will look something like this:

```javascript
{
  "cloudProvider": "GCP",
  "region": "us-west1-c",
  "vmType": "e2-standard-2",
  "credentials": {
    "privateKey": "-----BEGIN PRIVATE KEY-----\...\n-----END PRIVATE KEY-----\n",
    "clientEmail": "#########-compute@developer.gserviceaccount.com",
    "projectId": "my-gcp-project"
  }
}
```

## Azure

The schema for an Azure deployment config is as follows:

```javascript
{
  "cloudProvider": "string",
  "region": "string",
  "vmType": "string",
  "credentials": {
    "clientId": "string",
    "secret": "string",
    "subscriptionId": "string",
    "domain": "string"
  }
}
```

Take a look at the Azure [credentials]() and grab your `applicationId`, `secret`, `subscriptionId` and `directoryId`. An example of an Azure cloud configuration will look something like this:

```javascript
{
  "cloudProvider": "Azure",
  "region": "westus2",
  "vmType": "Standard_B1ls",
  "credentials": {
    "applicationId": "########-####-####-####-############",
    "secret": "##################################",
    "subscriptionId": "########-####-####-####-############",
    "directoryId": "########-####-####-####-############"
  }
}
```

