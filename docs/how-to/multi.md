# Deploy an app to multiple clouds and/or regions

AnyCloud makes it trivial to deploy a singular logical cluster, or application, to multiple regions and/or multiple clouds at the same time. AnyCloud will always keep at least one running server in each region/cloud defined.

## Multiple regions

Add an additional object to the existing deployment configuration in the `anycloud.json` that you want to deploy to multiple regions:

```javascript
{
  "staging": [
    {
      "credentials": "mystartup-aws",
      "region": "us-west-1",
      "vmType": "t3.medium",
    },
    {
      "credentials": "mystartup-aws",
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
      "credentials": "mystartup-aws",
      "region": "us-west-1",
      "vmType": "t3.medium",
    },
    {
      "credentials": "mystartup-gcp",
      "region": "us-west1-c",
      "vmType": "e2-medium"
    }
  ],
  ...
}
```