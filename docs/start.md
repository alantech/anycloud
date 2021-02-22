# Get started

Anycloud deployments are configured using a local file in `~/.anycloud/deploy.json`.
Each deployment will be keyed by a name and contain an array of cloud provider configurations (with credentials, region, etc.).


The schema for the `~/.anycloud/deploy.json` is as follows:

```
[
  "string": [
    {
      ...
    }
  ],
  "string": [
    {
      ...
    },
    {
      ...
    }
  ]
  ...
]
```

Each cloud provider configuration within the deployment array has a different configuration schema. 
It is possible to have multi-cloud and/or multi-region deployments by having multiple cloud provider configurations per deployment.
`~/.anycloud/deploy.json` could look something like:

```
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

```
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

Follow the steps at [credentials](./credentials#AWS) to get the `accessKeyId` and `secretAccessKey`. An example would look like:

```
[
  ...
  {
    "cloudProvider": "AWS"
    "credentials": {
      "accessKeyId": "#####################",
      "secretAccessKey": "###################"
    },
    "region": "us-west-1",
  },
  ...
]
```

## GCP

The schema for a GCP deployment config is as follows:

```
{
  "cloudProvider": "string",
  "region": "string",
  "projectId": "string",
  "credentials": {
    "private_key": "string",
    "client_email": "string"
  }
}
```

Take a look at the exported JSON file from your GCP [credentials](./credentials#GCP) and grab your `project_id`, `private_key` and `client_email`.
An example would look like:

```
{
  "cloudProvider": "GCP",
  "region": "us-west1-c",
  "projectId": "alan-deploy",
  "credentials": {
    "private_key": "-----BEGIN PRIVATE KEY-----\...\n-----END PRIVATE KEY-----\n",
    "client_email": "#########-compute@developer.gserviceaccount.com"
  }
}
```

