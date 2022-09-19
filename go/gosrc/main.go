package main

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"os"
	"time"

	"github.com/gorilla/mux"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

var client *mongo.Client

func main() {
	fmt.Println("Starting the application...")
	if os.Getenv("MONGODB_URI") == "" {
		log.Fatal("Missing the `MONGODB_URI` environment variable!")
	}
	fmt.Println("Connecting to the database...")
	client, err := mongo.NewClient(options.Client().ApplyURI(os.Getenv("MONGODB_URI")))
	if err != nil {
		log.Fatal(err)
	}
	ctx, _ := context.WithTimeout(context.Background(), 10*time.Second)
	err = client.Connect(ctx)
	if err != nil {
		log.Fatal(err)
	}
	defer client.Disconnect(ctx)
	fmt.Println("Linking the routers...")
	router := mux.NewRouter()
	fmt.Println("Serving at http://localhost:12345")
	http.ListenAndServe(":12345", router)
}
