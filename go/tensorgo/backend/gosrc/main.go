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
	"strconv"
	"time"

	"github.com/gorilla/handlers"
	"github.com/gorilla/mux"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

var client *mongo.Client
var collection *mongo.Collection

type ImageInformation struct {
	TensorflowLabels  []TensorflowLabelResult `json:"labels" bson:"labels"`
	RecognitionStatus string                  `json:"recognitionStatus" bson:"recognitionStatus"`
}

type TensorflowResult struct {
	error            any
	TensorflowLabels []TensorflowLabelResult
}

type TensorflowLabelResult struct {
	Label       string  `json:"label" bson:"label"`
	Probability float32 `json:"probability" bson:"probability"`
}

type UploadInformation struct {
	Id                string             `json:"_id" bson:"_id"`
	OriginalFilename  string             `json:"filename" bson:"filename"`
	UploadStatus      string             `json:"uploadStatus" bson:"uploadStatus"`
	RecognitionStatus string             `json:"recognitionStatus" bson:"recognitionStatus"`
	CreatedAt         primitive.DateTime `json:"createdAt" bson:"createdAt"`
}

type PersistenceInformation struct {
	Url          string `json:"url" bson:"url"`
	UploadStatus string `json:"uploadStatus" bson:"uploadStatus"`
}

type FileInfo struct {
	Id                string                  `json:"_id" bson:"_id"`
	UploadStatus      string                  `json:"uploadStatus" bson:"uploadStatus"`
	RecognitionStatus string                  `json:"recognitionStatus" bson:"recognitionStatus"`
	Url               string                  `json:"url" bson:"url"`
	OriginalFilename  string                  `json:"filename" bson:"filename"`
	TensorflowLabels  []TensorflowLabelResult `json:"labels" bson:"labels"`
	CreatedAt         primitive.DateTime      `json:"createdAt" bson:"createdAt"`
}

/*
 * SEND IMAGE TO TENSORFLOW SERVICE AND ADD RESULT TO MONGODB. SAVE IMAGE TO FILESYSTEM.
 */
// func UploadImageEndpoint(response http.ResponseWriter, request *http.Request) {
// 	response.Header().Set("content-type", "application/json")
// 	file, header, err := request.FormFile("image")
// 	if err != nil {
// 		response.WriteHeader(http.StatusInternalServerError)
// 		response.Write([]byte(`{ "message": "` + err.Error() + `" }`))
// 		return
// 	}
// 	defer file.Close()
// 	// STRETCH GOAL: TENSORFLOW COMPLETE, UPLOAD IMAGE TO S3
// 	diskFileName, err := saveFile(header.Filename, file)
// 	if err != nil {
// 		response.WriteHeader(http.StatusInternalServerError)
// 		response.Write([]byte(`{ "message": "` + err.Error() + `" }`))
// 		return
// 	}
// 	data, err := uploadFile("http://"+os.Getenv("TENSORFLOW_HOST")+":8080/recognize", header.Filename, file)
// 	if err != nil {
// 		response.WriteHeader(http.StatusInternalServerError)
// 		response.Write([]byte(`{ "message": "` + err.Error() + `" }`))
// 		return
// 	}
// 	var tensorflowResult ImageInformation
// 	_ = json.NewDecoder(bytes.NewReader(data)).Decode(&tensorflowResult)
// 	tensorflowResult.DiskFilename = diskFileName
// 	_, err = collection.InsertOne(context.Background(), tensorflowResult)
// 	if err != nil {
// 		response.WriteHeader(http.StatusInternalServerError)
// 		response.Write([]byte(`{ "message": "` + err.Error() + `" }`))
// 		return
// 	}
// 	response.Write(data)
// 	return
// }

func UploadImageEndpoint(response http.ResponseWriter, request *http.Request) {
	response.Header().Set("content-type", "application/json")
	file, header, err := request.FormFile("image")
	if err != nil {
		response.WriteHeader(http.StatusInternalServerError)
		response.Write([]byte(`{ "message": "` + err.Error() + `" }`))
		return
	}
	defer file.Close()

	id := primitive.NewObjectID().Hex()
	saveFileName := id + filepath.Ext(header.Filename)
	createdAt := primitive.NewDateTimeFromTime(time.Now())
	uploadInfo := UploadInformation{id, header.Filename, "in progress", "in progress", createdAt}

	_, err = collection.InsertOne(context.Background(), uploadInfo)
	if err != nil {
		// defer file.Close()
		response.WriteHeader(http.StatusInternalServerError)
		response.Write([]byte(`{ "message": "` + err.Error() + `" }`))
		return
	}
	json.NewEncoder(response).Encode(uploadInfo)

	// STRETCH GOAL: TENSORFLOW COMPLETE, UPLOAD IMAGE TO S3
	saveFile(id, saveFileName, file)
	// go processFile(id, header.Filename, file)

	data, err := uploadFile("http://"+os.Getenv("TENSORFLOW_HOST")+":8080/recognize", header.Filename, file)
	if err != nil {
		fmt.Println("TENSORFLOW ERR", err)
		return
	}
	var tensorflowResult ImageInformation
	_ = json.NewDecoder(bytes.NewReader(data)).Decode(&tensorflowResult)
	tensorflowResult.RecognitionStatus = "completed"
	filter := bson.M{"_id": id}
	update := bson.M{"$set": tensorflowResult}
	_, err = collection.UpdateOne(context.Background(), filter, update)
	if err != nil {
		fmt.Println("RECOGNITION UPDATE ERR", err)
		return
	}

	return
}

/*
 * RETURN ALL DOCUMENTS FROM MONGODB
 */
func FindImagesEndpoint(response http.ResponseWriter, request *http.Request) {
	query := request.URL.Query()
	filter := bson.M{}
	var threshold float64 = 0
	if query.Has("threshold") {
		if t, err := strconv.ParseFloat((query.Get("threshold")), 32); err == nil {
			threshold = t
		}
	}
	if query.Has("label") {
		filter = bson.M{"labels": bson.M{"$elemMatch": bson.M{"label": query.Get("label"), "probability": bson.M{"$gte": threshold}}}}
	}
	for _, field := range []string{"uploadStatus", "recognitionStatus"} {
		if query.Has(field) {
			filter[field] = query.Get(field)
		}
	}

	response.Header().Set("content-type", "application/json")
	var images []FileInfo = make([]FileInfo, 0, 0)
	cursor, err := collection.Find(context.Background(), filter)
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

func saveFile(id string, filename string, file multipart.File) error {
	// defer file.Close()
	filter := bson.M{"_id": id}
	path := "/images/" + filename
	diskFile, err := os.OpenFile("."+path, os.O_WRONLY|os.O_CREATE, os.ModePerm)
	if err != nil {
		go collection.UpdateOne(context.Background(), filter, bson.M{"$set": PersistenceInformation{"", "failed"}})
		return err
	}
	defer diskFile.Close()
	_, err = io.Copy(diskFile, file)
	if err != nil {
		go collection.UpdateOne(context.Background(), filter, bson.M{"$set": PersistenceInformation{"", "failed"}})
		return err
	}
	file.Seek(0, io.SeekStart) // TODO: what does this do?
	// return fmt.Sprintf("%x", md5.Sum([]byte(filename))), nil

	update := bson.M{"$set": PersistenceInformation{path, "completed"}}
	_, err = collection.UpdateOne(context.Background(), filter, update)
	if err != nil {
		// TODO: handle cleanup
		return err
	}
	return nil
}

func processFile(id string, filename string, file multipart.File) error {
	filter := bson.M{"_id": id}
	data, err := uploadFile("http://"+os.Getenv("TENSORFLOW_HOST")+":8080/recognize", filename, file)
	if err != nil {
		go collection.UpdateOne(context.Background(), filter, bson.M{"$set": ImageInformation{nil, "failed"}})
		return err
	}
	var tensorflowResult ImageInformation
	json.NewDecoder(bytes.NewReader(data)).Decode(&tensorflowResult)
	tensorflowResult.RecognitionStatus = "completed"
	fmt.Println("res1", tensorflowResult)

	// TODO: how to read error from tensorflowResult??
	// if tensorflowResult.error != nil {
	// 	go collection.UpdateOne(context.Background(), filter, bson.M{"$set": ImageInformation{nil, "failed"}})
	// 	return err
	// }

	update := bson.M{"$set": tensorflowResult}
	_, err = collection.UpdateOne(context.Background(), filter, update)
	if err != nil {
		// TODO: handle cleanup
		return err
	}
	return nil
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
	log.Fatal(http.ListenAndServe(":12345", handlers.CORS(handlers.AllowedHeaders([]string{"X-Requested-With", "Content-Type", "Authorization"}), handlers.AllowedMethods([]string{"GET", "POST", "PUT", "HEAD", "OPTIONS"}), handlers.AllowedOrigins([]string{"*"}))(router)))
}
