# Deploy an app to multiple clouds and/or regions

## Multiple regions

Add an additional object to the existing deployment configuration in the `anycloud.json` that you want to deploy to multiple regions:

```javascript
{
  "staging": [
    {
      "credentials": "piedpiper-aws",
      "region": "us-west-1",
      "vmType": "t3.medium",
    },
    {
      "credentials": "piedpiper-aws",
      "region": "us-east-1",
      "vmType": "t3.medium"
    }
  ],
  ...
}
```

## Multiple clouds

Add an additional object to the existing deployment configuration in the `anycloud.json` that you want to deploy to multiple cloud providers:

```javascript
{
  "staging": [
    {
      "credentials": "piedpiper-aws",
      "region": "us-west-1",
      "vmType": "t3.medium",
    },
    {
      "credentials": "piedpiper-gcp",
      "region": "us-west1-c",
      "vmType": "e2-medium"
    }
  ],
  ...
}
```