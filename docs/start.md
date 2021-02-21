# Get started

Anycloud deployments are configured by creating a local file in `~/.anycloud/deploy.json`
that will contain cloud provider configuration (credentials, region, etc.) for each deployment type.

Please define a json file with the following schema:
```
{
  "string": {
    "cloudProvider": "string",
    "region": "string",
    "credentials": {
      "accessKeyId": "string",
      "secretAccessKey": "string",
    }
  },
}
```
The deployment configuration is keyed by a string that represents the name for each deployment.

## AWS

1. To create an AWS access key follow this tutorial: https://aws.amazon.com/premiumsupport/knowledge-center/create-access-key/

2. Enable programmatic access for the IAM user, and attach the built-in 'AdministratorAccess' policy to your IAM user.

3. Configure an Anycloud deployment. The schema for the AWS deployment config is as follows:

```
{
  "cloudProvider": "AWS",
  "region": "us-west-2",
  "credentials": {
    "accessKeyId": "string",
    "secretAccessKey": "string",
  }
}
```

## GCP