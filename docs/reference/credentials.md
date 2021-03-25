# Credentials

AnyCloud deployments expect the cloud credentials to be configured using a local file that is not committed to any repository and is located at `~/.anycloud/credentials.json`. Each deployment will be keyed by a name and will provide the cloud provider configuration via two fields: `credentials` and `cloudProvider`. Possible values for `cloudProvider` are `AWS`, `GCP` and `Azure`. `credentials` will have a different schema depending on which cloud provider it is.

## AWS

```javascript
{
  "piedpiper-aws": {
    "cloudProvider": "AWS",
    "credentials": {
      "accessKeyId": "#####################",
      "secretAccessKey": "###################"
    }
  }
}
```

The top-level key is the alias you provide for referring to these credentials. For AWS the `cloudProvider` value is `AWS`, and in the credentials the `accessKeyId` and `secretAccessKey` come from an IAM user with an [`AmazonEC2FullAccess`](https://console.aws.amazon.com/iam/home#/policies/arn%3Aaws%3Aiam%3A%3Aaws%3Apolicy%2FAmazonEC2FullAccess)policy attached.

## GCP

```javascript
{
  "piedpiper-gcp": {
    "cloudProvider": "GCP",
    "credentials": {
      "privateKey": "-----BEGIN PRIVATE KEY-----\...\n-----END PRIVATE KEY-----\n",
      "clientEmail": "#########-compute@developer.gserviceaccount.com",
      "projectId": "my-gcp-project"
    }
  }
}
```

The top-level key is the alias you provide for referring to these credentials. For GCP the `cloudProvider` value is `GCP`, and in the credentials the `projectId` belongs to the GCP project that the service account is under. The`privateKey` and `clientEmail` come from a service account with the [`Compute Engine Admin`](https://cloud.google.com/compute/docs/access/iam#compute.admin) role.


## Azure

The top-level key is the alias you provide for referring to these credentials. For Azure the `cloudProvider` value is `Azure`. In the credentials the `directoryId` belongs to the Azure Active Directory, the `applicationId` and `secret` belong to the application and service principal under that application, and the `subscriptionId` belongs to the billing subscription.

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
}
```
