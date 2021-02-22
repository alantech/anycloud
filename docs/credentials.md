# Credentials

## AWS

1. To create an AWS access key follow this tutorial: https://aws.amazon.com/premiumsupport/knowledge-center/create-access-key/

2. Enable programmatic access for the IAM user, and attach the built-in `AdministratorAccess` policy to your IAM user.

## GCP

1. Create a service account for your GCP project as described [here](https://cloud.google.com/iam/docs/creating-managing-service-accounts#iam-service-accounts-create-console) with the `Compute Engine Admin role`.

2. Generate a service account key for your service account as described here and export it as a JSON file.