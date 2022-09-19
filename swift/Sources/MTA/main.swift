#! /usr/bin/env swift

import Foundation
import MongoSwift
import NIO

let elg = MultiThreadedEventLoopGroup(numberOfThreads: 4)

// // replace the following string with your connection uri
let uri = "mongodb://localhost:27017/test"

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

// print a list of database names
print(try client.listDatabaseNames().wait())

let db = client.db("mta")
let subways = db.collection("subways")

try subways.drop().wait()

var feedWithIds: [BSONDocument] = Array()
// iterate from i = 1 to 1 = 3
for _ in 0...10 {
    feedWithIds.append([
        "feedHeader": [:],
        "feedEntity": [
            "vehiclePosition": [
                "position": [
                    "latitude":  BSON.double(Double.random(in: 0.0 ..< 100.0)),
                    "longitude": BSON.double(Double.random(in: 0.0 ..< 100.0)),
                    "bearing":   BSON.double(Double.random(in: 0.0 ..< 100.0)),
                    "odometer":  BSON.double(Double.random(in: 0.0 ..< 100.0)),
                    "speed":     BSON.double(Double.random(in: 0.0 ..< 100.0))
                ]
            ]
        ]
    ] as BSONDocument)
}

print(try subways.insertMany(feedWithIds).wait())
