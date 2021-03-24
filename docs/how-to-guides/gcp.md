# Deploy to GCP

## Enable programmatic GCP access for AnyCloud

1. Create a service account for your GCP project as described [here](https://cloud.google.com/iam/docs/creating-managing-service-accounts#iam-service-accounts-create-console) with the [`Compute Engine Admin role`](https://cloud.google.com/compute/docs/access/iam#compute.admin).
2. Create a service account key for your newly service account as described [here](https://cloud.google.com/iam/docs/creating-managing-service-account-keys) and export it as a JSON file.
3. Add a new [credential](../reference/credentials.md) by taking the `privateKey`, `clientEmail`and `projectId`from step 2 and adding a new entry to your `~/.anycloud/credentials.json` file like this:

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
     ...
   }
   ```

## Configure your project

Define a new deployment configuration in the `anycloud.json` project you want to deploy to GCP like this:

```javascript
{
  "staging": [{
    "credentials": "piedpiper-gcp",
    "region": "us-west1-c",
    "vmType": "e2-medium"
  }],
  ...
}
```

We are referencing the previously defined credentials so make sure that the `credentials` value matches the key in `~/.anycloud/credentials.json`



