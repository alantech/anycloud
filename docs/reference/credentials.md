# Credentials

AnyCloud deployments expect the cloud credentials to be configured using a local file that is not committed to any repository and is located at `~/.anycloud/credentials.json`. Each deployment will be keyed by a name and will provide the cloud provider configuration via two fields:  `credentials` and `cloudProvider`. Possible values for `cloudProvider` are `AWS`, `GCP` and `Azure`. `credentials` will have a different schema depending on which cloud provider it is.

## AWS

The schema for the AWS deployment config is as follows:

```javascript
{
  "string": {
    "cloudProvider": "string",
    "credentials": {
      "accessKeyId": "string",
      "secretAccessKey": "string"
    }
  },
  ...
}
```

The `accessKeyId` and `secretAccessKey` are from an IAM user with . An example would look like:

```javascript
{
  "piedpiper-aws": {
    "cloudProvider": "AWS",
    "region": "us-west-1",
    "vmType": "t2.medium",
    "credentials": {
      "accessKeyId": "#####################",
      "secretAccessKey": "###################"
    },
  },
  ...
}
```

## GCP

The schema for a GCP deployment config is as follows:

```javascript
{
  "string": {
    "cloudProvider": "string",
    "credentials": {
      "privateKey": "string",
      "clientEmail": "string",
      "projectId": "string",
    }
  }
  ...
}
```

Take a look at the exported JSON file from your GCP [credentials](../how-to-guides/gcp.md) and grab your `project_id`, `private_key` and `client_email`. An example of a GCP cloud configuration will look something like this:

```javascript
{
  "piedpiper-gcp": {
    "cloudProvider": "GCP",
    "credentials": {
      "privateKey": "-----BEGIN PRIVATE KEY-----\...\n-----END PRIVATE KEY-----\n",
      "clientEmail": "#########-compute@developer.gserviceaccount.com",
      "projectId": "my-gcp-project"
    },
  },
  ...
}
```

## Azure

The schema for an Azure deployment config is as follows:

```javascript
{
  "string": {
    "cloudProvider": "string",
    "credentials": {
      "clientId": "string",
      "secret": "string",
      "subscriptionId": "string",
      "domain": "string"
    }
  }
}
```

Take a look at the Azure [credentials]() and grab your `applicationId`, `secret`, `subscriptionId` and `directoryId`. An example of an Azure cloud configuration will look something like this:

```javascript
{
  "piedpiper-azure": {
    "cloudProvider": "Azure",
    "credentials": {
      "applicationId": "########-####-####-####-############",
      "secret": "##################################",
      "subscriptionId": "########-####-####-####-############",
      "directoryId": "########-####-####-####-############"
    }
  }
  ...
}
```

