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

The `accessKeyId` and `secretAccessKey` come from an IAM user with an [`AmazonEC2FullAccess`](https://console.aws.amazon.com/iam/home#/policies/arn%3Aaws%3Aiam%3A%3Aaws%3Apolicy%2FAmazonEC2FullAccess)policy attached. An example credential entry would look like this:

```javascript
{
  "piedpiper-aws": {
    "cloudProvider": "AWS",
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

The `projectId` belongs to the GCP project that the service account is under. The`privateKey` and `clientEmail` come from a service account with the[`Compute Engine Admin`](https://cloud.google.com/compute/docs/access/iam#compute.admin) role. An example credential entry would look like this:

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

An example of an Azure cloud configuration will look something like this:

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
