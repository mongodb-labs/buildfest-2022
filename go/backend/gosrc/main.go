package main

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"io/ioutil"
	"log"
	"mime/multipart"
	"net/http"
	"os"
	"time"

	"github.com/gorilla/mux"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

var client *mongo.Client
var collection *mongo.Collection

type TensorflowClassifyResult struct {
	Filename string                  `json:"filename" bson:"filename"`
	Labels   []TensorflowLabelResult `json:"labels" bson:"labels"`
}

type TensorflowLabelResult struct {
	Label       string  `json:"label" bson:"label"`
	Probability float32 `json:"probability" bson:"probability"`
}

/*
 * SEND IMAGE TO TENSORFLOW SERVICE AND ADD RESULT TO MONGODB
 */
func UploadImageEndpoint(response http.ResponseWriter, request *http.Request) {
	response.Header().Set("content-type", "application/json")
	file, header, err := request.FormFile("image")
	if err != nil {
		response.WriteHeader(http.StatusInternalServerError)
		response.Write([]byte(`{ "message": "` + err.Error() + `" }`))
		return
	}
	defer file.Close()
	// STRETCH GOAL: TENSORFLOW COMPLETE, UPLOAD IMAGE TO S3
	_, err = saveFile(header.Filename, file)
	if err != nil {
		response.WriteHeader(http.StatusInternalServerError)
		response.Write([]byte(`{ "message": "` + err.Error() + `" }`))
		return
	}
	data, err := uploadFile("http://"+os.Getenv("TENSORFLOW_HOST")+":8080/recognize", header.Filename, file)
	if err != nil {
		response.WriteHeader(http.StatusInternalServerError)
		response.Write([]byte(`{ "message": "` + err.Error() + `" }`))
		return
	}
	var tensorflowResult TensorflowClassifyResult
	_ = json.NewDecoder(bytes.NewReader(data)).Decode(&tensorflowResult)
	_, err = collection.InsertOne(context.Background(), tensorflowResult)
	if err != nil {
		response.WriteHeader(http.StatusInternalServerError)
		response.Write([]byte(`{ "message": "` + err.Error() + `" }`))
		return
	}
	response.Write(data)
	return
}

func saveFile(filename string, file multipart.File) (bool, error) {
	diskFile, err := os.OpenFile("./images/"+filename, os.O_WRONLY|os.O_CREATE, os.ModePerm)
	if err != nil {
		return false, err
	}
	defer diskFile.Close()
	_, err = io.Copy(diskFile, file)
	if err != nil {
		return false, err
	}
	file.Seek(0, io.SeekStart)
	return true, nil
}

func uploadFile(url string, filename string, file multipart.File) ([]byte, error) {
	httpRequestBody := &bytes.Buffer{}
	formWriter := multipart.NewWriter(httpRequestBody)
	writer, err := formWriter.CreateFormFile("image", filename)
	if err != nil {
		return nil, err
	}
	_, err = io.Copy(writer, file)
	if err != nil {
		return nil, err
	}
	formWriter.Close()
	httpClient := &http.Client{}
	httpRequest, err := http.NewRequest("POST", url, bytes.NewReader(httpRequestBody.Bytes()))
	if err != nil {
		return nil, err
	}
	httpRequest.Header.Set("content-type", formWriter.FormDataContentType())
	httpResponse, err := httpClient.Do(httpRequest)
	if err != nil {
		return nil, err
	}
	data, _ := ioutil.ReadAll(httpResponse.Body)
	file.Seek(0, io.SeekStart)
	return data, nil
}

/*
 * START AND CONFIGURE MONGODB CONNECTION AND WEB SERVICE ROUTING
 */
func main() {
	fmt.Println("Starting the application...")
	if os.Getenv("MONGODB_URI") == "" {
		log.Fatal("Missing the `MONGODB_URI` environment variable!")
	}
	if os.Getenv("MONGODB_DATABASE") == "" {
		log.Fatal("Missing the `MONGODB_DATABASE` environment variable!")
	}
	if os.Getenv("MONGODB_COLLECTION") == "" {
		log.Fatal("Missing the `MONGODB_COLLECTION` environment variable!")
	}
	if os.Getenv("TENSORFLOW_HOST") == "" {
		os.Setenv("TENSORFLOW_HOST", "localhost")
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
	collection = client.Database(os.Getenv("MONGODB_DATABASE")).Collection(os.Getenv("MONGODB_COLLECTION"))
	defer func() {
		fmt.Println("Disconnecting from the database...")
		if err = client.Disconnect(ctx); err != nil {
			panic(err)
		}
	}()
	fmt.Println("Linking the routers...")
	router := mux.NewRouter()
	router.HandleFunc("/upload", UploadImageEndpoint).Methods("POST")
	router.PathPrefix("/images/").Handler(http.StripPrefix("/images/", http.FileServer(http.Dir("./images/"))))
	fmt.Println("Serving at http://localhost:12345")
	http.ListenAndServe(":12345", router)
}
