# Credentials

## AWS

1. To create an AWS access key follow this tutorial: [https://aws.amazon.com/premiumsupport/knowledge-center/create-access-key/](https://aws.amazon.com/premiumsupport/knowledge-center/create-access-key/)
2. Enable programmatic access for the IAM user, and attach the built-in [`AmazonEC2FullAccess`](https://console.aws.amazon.com/iam/home#/policies/arn%3Aaws%3Aiam%3A%3Aaws%3Apolicy%2FAmazonEC2FullAccess)policy to your IAM user.

## GCP

1. Create a service account for your GCP project as described [here](https://cloud.google.com/iam/docs/creating-managing-service-accounts#iam-service-accounts-create-console) with the [`Compute Engine Admin role`](https://cloud.google.com/compute/docs/access/iam#compute.admin).
2. Generate a service account key for your service account as described here and export it as a JSON file.



