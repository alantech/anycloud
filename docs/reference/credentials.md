# Credentials

AnyCloud supports managing multiple cloud `Credentials` via the `anycloud credentials` CLI command. The `Credentials` are stored in a local file that is not committed to any repository located at `~/.anycloud/credentials.json`. AnyCloud supports `Credentials` for AWS, GCP and Azure. Each `Credential` has a name, or alias, to refer to it within the `Deploy Configs`. This allows you to, for example, create credentials for your personal AWS and GCP accounts as well as for a company's AWS account and use each of them to create separate `Deploy Configs` for different `Apps` or create a multi region/cloud `Deploy Config` for the same `App`.

## AWS

An AWS Credential consists of an `accessKeyId` and `secretAccessKey` from an IAM user with an [`AmazonEC2FullAccess`](https://console.aws.amazon.com/iam/home*/policies/arn%3Aaws%3Aiam%3A%3Aaws%3Apolicy%2FAmazonEC2FullAccess) policy attached.

```
$ anycloud credential add
Pick cloud provider for the new Credential:
> AWS
  GCP
  Azure
Name for new Credential: mystartup-aws
AWS Access Key ID: ******************
AWS Secret Access Key: ******************
Successfully created "mystartup-aws" Credential
```

## GCP

A GCP Credential consists of an `privateKey` and `clientEmail` that come from a service account with the [`Compute Engine Admin`](https://cloud.google.com/compute/docs/access/iam*compute.admin) role and the `projectId` in which the service account is contained.

```
$ anycloud credential add
Pick cloud provider for the new Credential:
  AWS
> GCP
  Azure
Credential Name: mystartup-gcp
GCP Project ID: my-gcp-project
GCP Client Email: *******-compute@developer.gserviceaccount.com
GCP Private Key: -----BEGIN PRIVATE KEY-----\*****\n-----END PRIVATE KEY-----\n
Successfully created "mystartup-gcp" Credential
```

## Azure

An Azure Credential consists of the `directoryId` that belongs to the [Azure Active Directory](https://docs.microsoft.com/en-us/azure/active-directory/fundamentals/active-directory-whatis), the `applicationId` and `secret` of the [application and service principal](https://docs.microsoft.com/en-us/azure/active-directory/develop/app-objects-and-service-principals), and the `subscriptionId` of the [billing subscription](https://docs.microsoft.com/en-us/azure/active-directory/fundamentals/active-directory-how-subscriptions-associated-directory).

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
