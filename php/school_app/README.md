# Buildfest 2022 PHP Project

## How to run the app locally

1. If you want to use CSFLE, download crypt_shared library for Debian 10, and put `mongo_crypt_v1.so` file in the project folder.
1. Build the container: `docker build -t school_app .`
2. Run the app: `docker run -v $(pwd)/src:/school_app/src --network="host" -e MONGODB_URI='mongodb+srv://your_atlas_cluster/' -e DATABASE='school_app' school_app`
3. Open [http://localhost:8080](http://localhost:8080).
