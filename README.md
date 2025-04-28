# StartProx

This is just a simple utility that can help start `cloud-sql-proxy` to connect to sql instances
on google cloud platform. It will allow users to configure a list of instances to connect to
and allow users to connect to a default port value.

## Installation

Go to the latest releases, and download the version for your OS and CPU architecture.
You may need to add executable permissions:

```
chmod +x startprox-<rest_of_file_name>
```

Then just run the program like usual.

```
./startprox
```

StartProx will look for a config in your `$HOME/.config/startprox/` folder.
The application will also prompt you to create a default `config.yml` if not found.
Make sure you edit it to include the information about your own cloudsql instances
or you will not be able to connect.
