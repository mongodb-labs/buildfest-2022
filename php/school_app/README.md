# Buildfest 2022 PHP Project

## How to run the app locally

1. Build the container: `docker build -t school_app .`
2. Run the app: `docker run -v $(pwd)/src:/school_app/src -p 8080:80 -e MONGODB_URI='mongodb+srv://your_atlas_cluster/' -e DATABASE='school_app' school_app`
3. Open [http://localhost:8080](http://localhost:8080).
