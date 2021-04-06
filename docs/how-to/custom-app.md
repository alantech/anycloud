# Define a custom app ID

To create a new AnyCloud app with a custom name simply pass the optional `-a`, or `--app-id` parameter.

```bash
$ anycloud new <deploy-name> -a <app-id>
▇ Creating new app

```

Remember `<deploy-name>` must be equal to one of the keys defined in your `anycloud.json`. Also if the provided app ID is already defined you will not be able to create the new app.