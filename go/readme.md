## INSTRUCTIONS [DOCKER]

1. Duplicate the **docker-compose.yml.example** file and rename the new file **docker-compose.yml** so it doesn't get included in Git.
2. Add the MongoDB Atlas URI string into the `MONGODB_URI` field of the **docker-compose.yml** file.
3. Build and deploy the Go application with `DOCKER_BUILDKIT=0 docker-compose up` using the optional `-d` flag if you'd like to deploy in detached mode.

Two containers will be deployed with the Compose file. The first is the Go web service:

```
curl localhost:12345/upload -F 'image=@./nraboy.jpeg'
```

The Go web service will communicate to MongoDB as well as the Tensorflow container. The second container is the Tensorflow application:

```
curl localhost:8080/recognize -F 'image=@./nraboy.jpeg'
```

Making requests to the Tensorflow application directly will skip the MongoDB and other backend related logic.

**NOTE:** The `DOCKER_BUILDKIT=0` variable is required to build from a remote Git repository. The default has a bug that will throw errors if not explicitly set.

## TEAM

- Anaiya Raisinghani
- Daria Pardue
- Sourabh Bagrecha
- Dominic Frei
- Nic Raboy