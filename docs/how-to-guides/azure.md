# Deploy to Azure

##  Enable programmatic Azure access for AnyCloud

1. Create an Azure Active Directory application and its Service Principal as described [here](https://docs.microsoft.com/en-us/azure/active-directory/develop/howto-create-service-principal-portal). After following the previous instructions you can copy your `Application (client) ID` and your `Directory (tenant) ID`.
2. Create a secret for the service principal created above as described [here](https://docs.microsoft.com/en-us/azure/active-directory/develop/howto-create-service-principal-portal#option-2-create-a-new-application-secret). Copy the secret `Value` immediately since you will not be able to retrieve it again later.
3. For the subscription ID you can go to your [subscriptions page](https://portal.azure.com/#blade/Microsoft_Azure_Billing/SubscriptionsBlade) in Azure portal and get the ID.
4. To be able to use Anycloud with Azure you will need to manage your subscription resource provider registration as described [here](https://docs.microsoft.com/en-us/azure/azure-resource-manager/templates/error-register-resource-provider#solution-3---azure-portal). You will need to register: `Microsoft.Compute`, `Microsoft.Network`, `Microsoft.Storage` and `Microsoft.Security`.
5. Add a new [credential](../reference/credentials.md) by taking the `privateKey`, `clientEmail`and `projectId`from step 2 and adding a new entry to your `~/.anycloud/credentials.json` file like this:

   ```javascript
   {
     "piedpiper-azure": {
       "cloudProvider": "Azure",
       "credentials": {
         "applicationId": "########-####-####-####-############",
         "secret": "##################################",
         "subscriptionId": "########-####-####-####-############",
         "directoryId": "########-####-####-####-############"
       }
     }
     ...
   }
   ```

## **Configure your project**

Define a new deployment configuration in the `anycloud.json` project you want to deploy to Azure like this:

```javascript
{
  "staging": [{
    "credentials": "piedpiper-azure",
    "region": "westus2",
    "vmType": "Standard_B1ls",
  }],
  ...
}
```

We are referencing the previously defined credentials so make sure that the `credentials` value matches the key in `~/.anycloud/credentials.json`
