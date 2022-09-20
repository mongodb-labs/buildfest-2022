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


signal(SIGINT, SIG_IGN) // // Make sure the signal does not terminate the application.

let sigintSrc = DispatchSource.makeSignalSource(signal: SIGINT, queue: .main)
sigintSrc.setEventHandler {
    print("Stopping MTA sink! Thank you for riding with us... Mind the Gap")
    exit(0)
}


let db = client.db("mta")
let subways = db.collection("subways")

let mtaBDFMLineURL = "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-bdfm"

func myRequest(){
    var request = URLRequest(url: URL(string: mtaBDFMLineURL)!);
    request.httpMethod = "GET"
    request.setValue(API_KEY, forHTTPHeaderField: <#T##String#>)
    let semaphore: DispatchSemaphore = DispatchSemaphore(value: 0)
    let task = URLSession.shared.dataTask(with: request) { (data, response, error) in
        // get responce here
        // Use response
        print("2")
        semaphore.signal()
    }
    task.resume()
    print("1")
    semaphore.wait()
    print("3")
    return 
}

while (true) {
    
    sleep(1000)
    // http request
    // deocde the protobuf
    // BSONDocument -> insert to mongodb
    // ??? Timeseries - improvement
    
    let decodedInfo = try TransitRealtime_FeedMessage(serializedData: binaryData)
    // insert

}
