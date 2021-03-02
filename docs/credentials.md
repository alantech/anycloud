# Credentials

## AWS

1. Create a new AWS access key as described [here](https://aws.amazon.com/premiumsupport/knowledge-center/create-access-key/)
2. Enable programmatic access for a new IAM user, and attach the built-in [`AmazonEC2FullAccess`](https://console.aws.amazon.com/iam/home#/policies/arn%3Aaws%3Aiam%3A%3Aaws%3Apolicy%2FAmazonEC2FullAccess)policy to it.

## GCP

1. Create a service account for your GCP project as described [here](https://cloud.google.com/iam/docs/creating-managing-service-accounts#iam-service-accounts-create-console) with the [`Compute Engine Admin role`](https://cloud.google.com/compute/docs/access/iam#compute.admin).
2. Create a service account key for your newly service account as described [here](https://cloud.google.com/iam/docs/creating-managing-service-account-keys) and export it as a JSON file.

## Azure

1. Create an Azure Active Directory application and it's Service Principal as described [here](https://docs.microsoft.com/en-us/azure/active-directory/develop/howto-create-service-principal-portal). After following the previous instructions you can copy your `Application (client) ID` and your `Directory (tenant) ID`.

2. Create a secret for the service principal created above as described [here](https://docs.microsoft.com/en-us/azure/active-directory/develop/howto-create-service-principal-portal#option-2-create-a-new-application-secret). Copy the secret `Value` immediately since you will not be able to retrieve it again later.

3. For the subscription ID you can go to your [subscriptions page](https://portal.azure.com/#blade/Microsoft_Azure_Billing/SubscriptionsBlade) in Azure portal and get the ID.

4. To be able to use Anycloud with Azure you will need to manage your subscription resource provider registration as described [here](https://docs.microsoft.com/en-us/azure/azure-resource-manager/templates/error-register-resource-provider#solution-3---azure-portal). You will need to register: `Microsoft.Compute`, `Microsoft.Network`, `Microsoft.Storage` and `Microsoft.Security`.
