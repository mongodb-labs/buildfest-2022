#! /usr/bin/env swift

import Foundation
import MongoSwift
import NIO

let elg = MultiThreadedEventLoopGroup(numberOfThreads: 4)

// // replace the following string with your connection uri
let uri = ProcessInfo.processInfo.environment["MONGODB_URI"] ?? "mongodb://localhost:27017,localhost:27018,localhost:27019/mta?replicaSet=rep10"

// replace the following string with your connection uri

let client = try MongoClient(
    uri,
    using: elg
)

defer {
    // clean up driver resources
    try? client.syncClose()
    cleanupMongoSwift()

    // shut down EventLoopGroup
    try? elg.syncShutdownGracefully()
}

let dateFormatter : DateFormatter = DateFormatter()
dateFormatter.dateFormat = "yyyy-MM-dd HH:mm:ss"

let mtaBDFMLineURL = "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-bdfm"

guard let MTA_API_KEY = ProcessInfo.processInfo.environment["MTA_API_KEY"] else {
    print("Must provide MTA_API_KEY env variable to continue!")
    exit(1)
}

func requestMTA() -> Data? {
    var request = URLRequest(url: URL(string: mtaBDFMLineURL)!);
    request.httpMethod = "GET"
    request.setValue(MTA_API_KEY, forHTTPHeaderField: "x-api-key")
    let semaphore: DispatchSemaphore = DispatchSemaphore(value: 0)
    var responseData: Data? = nil
    let task = URLSession.shared.dataTask(with: request) { (data: Data?, response, error) in
        // get responce here
        // Use response
        responseData = data
        semaphore.signal()
    }
    task.resume()
    semaphore.wait()
    // Safe to use responseData
    return responseData
}

let db = client.db("mta")
let subways = db.collection("feedMessages")

var sleepSeconds: UInt32 = 120
if CommandLine.arguments.count > 1 {
    sleepSeconds = UInt32(CommandLine.arguments[1]) ?? 120
}

var occurrences = 0
while (occurrences < 1000) {
    // Hard cap at 1000 runs just cus we probably won't demo that long

    let dateString = dateFormatter.string(from: Date())
    print("[\(dateString)] trying to request MTA")
    // http request
    if let response = requestMTA() {
        print("[\(dateString)] success! MTA responded")
        // deocde the protobuf
        let decodedInfo = try TransitRealtime_FeedMessage(serializedData: response)
        print("[\(dateString)] decoded the protobuf")

        let protobufJSON = try String(decoding: decodedInfo.jsonUTF8Data(), as: UTF8.self)
        print("[\(dateString)] stringified the proto to JSON")

        let mongodbDoc = try BSONDocument(fromJSON:protobufJSON)
        print("[\(dateString)] BSON decoding fromJSON")

        // BSONDocument -> insert to mongodb
        print("[\(dateString)] trying insertOne to \(subways.namespace)")
        let insertRes = try subways.insertOne(mongodbDoc).wait()
        print("[\(dateString)] completed insertOne to \(subways.namespace): \(String(describing: insertRes))")
    } else {
        print("[\(dateString)] failure! MTA did not respond")
    }

    print("[\(dateString)] sleeping for \(sleepSeconds) seconds")
    sleep(sleepSeconds)
    print("[\(dateString)] waking up!")
    occurrences += 1
    print("[\(dateString)] occurrences = \(occurrences)")
}

print("Stopping MTA sink! Thank you for riding with us... Mind the Gap")
exit(0)
