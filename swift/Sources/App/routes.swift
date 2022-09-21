import Vapor
import MongoDBVapor

let nealController = NealController()
let trainsController = TrainsController()

func routes(_ app: Application) throws {

  let file = FileMiddleware(publicDirectory: app.directory.publicDirectory)
  app.middleware.use(file)

  app.get { req async in
    "It works!"
  }


  app.get("feed-messages") { req async throws -> [BSONDocument] in
    try await req.findFeedMessages()
  }

  app.webSocket("feed") { req, ws async in
    try? await req.feed(ws: ws)
  }

  try? nealController.boot(routes: app.routes)
  try? trainsController.boot(routes: app.routes)
}

extension Request {
  var feedMessageCollection: MongoCollection<BSONDocument> {
    self.application.mongoDB.client.db("mta").collection("feedMessagesLirr")
  }

  func findFeedMessages() async throws -> [BSONDocument] {
    do {
      return try await self.feedMessageCollection.find().toArray()
    } catch {
      throw Abort(.internalServerError, reason: "Failed to load feed messages: \(error)")
    }
  }

  func feed(ws: WebSocket) async throws {
    try await ws.send("{\"hello\": 1}")
    print("ws connected")

    let encoder: ExtendedJSONEncoder = ExtendedJSONEncoder()
    let changeStreamTask = Task {
      let pipeline: [BSONDocument] = [
        ["$addFields": ["fullDocument._id": "$_id"]],
        ["$replaceRoot": ["newRoot": "$fullDocument"]],
        ["$project": ["entities": ["$filter": ["input": "$entity", "as": "ent", "cond": ["$not": "$$ent.tripUpdate"]]]]]
      ]
      print(pipeline)
      for try await event in try await feedMessageCollection.watch(pipeline, withEventType: BSONDocument.self) {
        let jsonString: String = String(decoding: try encoder.encode(event), as: UTF8.self)
        print(jsonString)
        try await ws.send(jsonString)
      }
    }
    
    let result = await changeStreamTask.result
    print(result)


    ws.onClose.whenComplete { result in
      changeStreamTask.cancel()
    }
  }
}
