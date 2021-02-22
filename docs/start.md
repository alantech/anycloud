# Get Started

Anycloud deployments are configured using a local file in `~/.anycloud/deploy.json`. Each deployment will be keyed by a name and contain an array of cloud provider configurations with `credentials`, `region` and `cloudProvider`. Possible values for `cloudProvider` are [`AWS`](start.md#aws) and [`GCP`](start.md#gcp) and the other two fields will have a different possible values depending on which cloud provider it is.

The schema for the `~/.anycloud/deploy.json` is as follows:

```javascript
{
  "string": [
    {
      "cloudProvider": "string",
      "region": "string",
      "credentials" {
        ...
      }
    }
  ],
  "string": [
    {
      "cloudProvider": "string",
      "region": "string",
      "credentials" {
        ...
      }
    },
    {
      "cloudProvider": "string",
      "region": "string",
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
      "credentials": {
        ...
      },
    },
    {
      "cloudProvider": "AWS",
      "region": "us-west-2",
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
    "credentials": {
      "accessKeyId": "string",
      "secretAccessKey": "string",
    }
  }
  ...
]
```

Follow the steps at [credentials](credentials.md#aws) to get the `accessKeyId` and `secretAccessKey`. An example would look like:

```javascript
[
  ...
  {
    "cloudProvider": "AWS",
    "region": "us-west-1",
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
  "credentials": {
    "privateKey": "string",
    "clientEmail": "string",
    "projectId": "string",
  }
}
```

Take a look at the exported JSON file from your GCP [credentials](credentials.md#gcp) and grab your `project_id`, `private_key` and `client_email`. An example of a GCP cloud configuration will look something like this:

```javascript
{
  "cloudProvider": "GCP",
  "region": "us-west1-c",
  "credentials": {
    "privateKey": "-----BEGIN PRIVATE KEY-----\...\n-----END PRIVATE KEY-----\n",
    "clientEmail": "#########-compute@developer.gserviceaccount.com",
    "projectId": "my-gcp-project"
  }
}
```

