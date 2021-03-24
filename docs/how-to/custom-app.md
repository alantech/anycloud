# Define a custom app ID

To create a new AnyCloud app with a custom name simple pass the optional `-a`, or `--app-id` parameter

```
$ anycloud new staging -a test
▇ Creating new app

```

If the provided app ID is already defined you will not be able to create the new app:

```
$ anycloud new staging -a test
▇ Creating new app
Failed to create a new app. Error: Another application with same app ID already exists.
```