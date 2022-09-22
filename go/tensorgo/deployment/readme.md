# Deployment

## Run

The deployment script can be run as follows:
```bash
python3 deploy.py ~/.ssh/buildfest-2022-go.pem
```

The `.pem` will be shared with you privately.

# Process

The deployment script is a simplified version specifically for buildfest.
It does not actually copy data to the server but instead uses key forwarding and 
simply checks out the repository on the server.

It then creates a new `mongo.service` file based on the jinja2 template file 
`mongo.service.j2` that registers the application as a service, including 
automatic restart of the application if it fails. It also automatically starts
the application on reboot of the server.