# Define a custom app name

To create a new AnyCloud app with a custom name simple pass the optional `-a` parameter

```
$ anycloud new staging -a test
▇ Creating new app

```

If the provided app name, or id, is already defined you will not be able to create the new app:

```
$ anycloud new staging -a test
▇ Creating new app
Failed to create a new app. Error: Another application with same App Id already exists.
```