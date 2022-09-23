#! /usr/bin/env swift

/*
    This script periodically reads LIRR Train GTFS data from MTA feed
    and inserts it into MongoDB.

    Set the following variables to match your dev environment:
    - uri: currently set to read your MONGODB_URI variable or otherwise default to something else
    - feedUrl: url for the MTA GTFS feed from which to get data
    - MTA_API_KEY: API key for MTA feeds
    - dbName: name of the database in which to store data
    - collName: name of the collection in which to store data
*/

import Foundation
import MongoSwift
import NIO

// Make sure the following variables are set for your dev environment:
let uri = ProcessInfo.processInfo.environment["MONGODB_URI"]!
let feedUrl = "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/lirr%2Fgtfs-lirr"
guard let MTA_API_KEY = ProcessInfo.processInfo.environment["MTA_API_KEY"] else {
    print("Must provide MTA_API_KEY env variable to continue!")
    exit(1)
}
let dbName = "mta"
let collName = "feedMessagesLirr"
// -------------------------------------------------------------------

let elg = MultiThreadedEventLoopGroup(numberOfThreads: 4)
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

func requestMTA() -> Data? {
    var request = URLRequest(url: URL(string: feedUrl)!);
    request.httpMethod = "GET"
    request.setValue(MTA_API_KEY, forHTTPHeaderField: "x-api-key")
    let semaphore: DispatchSemaphore = DispatchSemaphore(value: 0)
    var responseData: Data? = nil
    let task = URLSession.shared.dataTask(with: request) { (data: Data?, response, error) in
        // get response here
        // Use response
        responseData = data
        semaphore.signal()
    }
    task.resume()
    semaphore.wait()
    // Safe to use responseData
    return responseData
}

let db = client.db(dbName)
let coll = db.collection(collName)

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
        print("[\(dateString)] trying insertOne to \(coll.namespace)")
        let insertRes = try coll.insertOne(mongodbDoc).wait()
        print("[\(dateString)] completed insertOne to \(coll.namespace): \(String(describing: insertRes))")

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
