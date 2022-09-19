#! /usr/bin/env swift

import MongoSwift

import MongoSwift
import NIO

let elg = MultiThreadedEventLoopGroup(numberOfThreads: 4)

// replace the following string with your connection uri
let uri = "mongodb://localhost:27017/test"

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

// your application logic
