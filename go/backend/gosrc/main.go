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
	"path/filepath"
	"time"

	"github.com/gorilla/mux"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

var client *mongo.Client
var collection *mongo.Collection

type ImageInformation struct {
	OriginalFilename string                  `json:"filename" bson:"filename"`
	DiskFilename     string                  `json:"disk_filename" bson:"disk_filename"`
	TensorflowLabels []TensorflowLabelResult `json:"labels" bson:"labels"`
}

type TensorflowLabelResult struct {
	Label       string  `json:"label" bson:"label"`
	Probability float32 `json:"probability" bson:"probability"`
}

/*
 * SEND IMAGE TO TENSORFLOW SERVICE AND ADD RESULT TO MONGODB. SAVE IMAGE TO FILESYSTEM.
 */
func UploadImageEndpoint(response http.ResponseWriter, request *http.Request) {
	fmt.Println("Running UploadImageEndpoint")
	response.Header().Set("content-type", "application/json")
	file, header, err := request.FormFile("image")
	if err != nil {
		response.WriteHeader(http.StatusInternalServerError)
		response.Write([]byte(`{ "message": "` + err.Error() + `" }`))
		return
	}
	defer file.Close()
	// STRETCH GOAL: TENSORFLOW COMPLETE, UPLOAD IMAGE TO S3
	diskFileName, err := saveFile(header.Filename, file)
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
	var tensorflowResult ImageInformation
	_ = json.NewDecoder(bytes.NewReader(data)).Decode(&tensorflowResult)
	tensorflowResult.DiskFilename = diskFileName
	_, err = collection.InsertOne(context.Background(), tensorflowResult)
	if err != nil {
		response.WriteHeader(http.StatusInternalServerError)
		response.Write([]byte(`{ "message": "` + err.Error() + `" }`))
		return
	}
	response.Write(data)
	return
}

/*
 * RETURN ALL DOCUMENTS FROM MONGODB
 */
func FindImagesEndpoint(response http.ResponseWriter, request *http.Request) {
	response.Header().Set("content-type", "application/json")
	var images []ImageInformation
	cursor, err := collection.Find(context.Background(), bson.M{})
	if err != nil {
		response.WriteHeader(http.StatusInternalServerError)
		response.Write([]byte(`{ "message": "` + err.Error() + `" }`))
		return
	}
	if err = cursor.All(context.Background(), &images); err != nil {
		response.WriteHeader(http.StatusInternalServerError)
		response.Write([]byte(`{ "message": "` + err.Error() + `" }`))
		return
	}
	json.NewEncoder(response).Encode(images)
}

func saveFile(filename string, file multipart.File) (string, error) {
	diskFile, err := os.OpenFile("./images/"+primitive.NewObjectID().Hex()+filepath.Ext(filename), os.O_WRONLY|os.O_CREATE, os.ModePerm)
	if err != nil {
		return "", err
	}
	defer diskFile.Close()
	_, err = io.Copy(diskFile, file)
	if err != nil {
		return "", err
	}
	file.Seek(0, io.SeekStart)
	// return fmt.Sprintf("%x", md5.Sum([]byte(filename))), nil
	return primitive.NewObjectID().Hex() + filepath.Ext(filename), nil
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
	fmt.Println("Purging old database collections...")
	if err = collection.Drop(ctx); err != nil {
		log.Fatal(err)
	}
	fmt.Println("Linking the routers...")
	router := mux.NewRouter()
	router.HandleFunc("/upload", UploadImageEndpoint).Methods("POST")
	router.HandleFunc("/find", FindImagesEndpoint).Methods("GET")
	router.PathPrefix("/images/").Handler(http.StripPrefix("/images/", http.FileServer(http.Dir("./images/"))))
	fmt.Println("Serving at http://localhost:12345")
	http.ListenAndServe(":12345", router)
}
