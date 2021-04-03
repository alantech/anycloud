# Deploy to Azure

## Enable programmatic Azure access for AnyCloud

1) Create an Azure Active Directory application and its Service Principal as described [here](https://docs.microsoft.com/en-us/azure/active-directory/develop/howto-create-service-principal-portal). After following the previous instructions you can copy your `Application (client) ID` and your `Directory (tenant) ID`.

2) Create a secret for the service principal created above as described [here](https://docs.microsoft.com/en-us/azure/active-directory/develop/howto-create-service-principal-portal#option-2-create-a-new-application-secret). Copy the secret `Value` immediately since you will not be able to retrieve it again later.

3) For the subscription ID you can go to your [subscriptions page](https://portal.azure.com/#blade/Microsoft_Azure_Billing/SubscriptionsBlade) in Azure portal and get the ID.

4) To be able to use Anycloud with Azure you will need to manage your subscription resource provider registration as described [here](https://docs.microsoft.com/en-us/azure/azure-resource-manager/templates/error-register-resource-provider#solution-3---azure-portal). You will need to register: `Microsoft.Compute`, `Microsoft.Network`, `Microsoft.Storage` and `Microsoft.Security`.

5) Add a new `Credential` by taking the values from the previous steps. You will need to pick a name or alias for the `Credential`. The initial value will be `azure`. In this example, we will call it `mystartup-azure`.

```
$ anycloud credential add
Pick cloud provider for the new Credential:
  AWS
  GCP
> Azure
Credential Name: mystartup-azure
Azure Application ID: ********-****-****-****-************
Azure Directory ID: ********-****-****-****-************
Azure Subscription ID: ********-****-****-****-************
Azure Secret: **********************************
Successfully created "mystartup-gcp" Credential
```

## **Configure your project**

Define a new `Deploy Config` in the `anycloud.json` project you want to deploy to Azure using the AnyCloud CLI:

```
$ anycloud config add
Name for new Deploy Config: staging
Pick Credential to use:
> mystartup-azure
Region name: westus2
Virtual Machine Type: Standard_B1ls
Do you want to add another region to this Deploy Config? n
Successfully created "staging" Deploy Config
```