import Vapor
import MongoDBVapor

func routes(_ app: Application) throws {

  let file = FileMiddleware(publicDirectory: app.directory.publicDirectory)
  app.middleware.use(file)

  app.get { req async in
    "It works!"
  }

  app.get("feed-messages") { req async throws -> [BSONDocument] in
    try await req.findFeedMessages()
  }
  
  app.get("stops") { req async throws -> [BSONDocument] in
    try await req.findStops()
  }

  app.webSocket("feed") { req, ws async in
    try? await req.feed(ws: ws)
  }
}

extension Request {
  var feedMessageCollection: MongoCollection<BSONDocument> {
    self.application.mongoDB.client.db("mta").collection("feedMessagesLirr")
  }
  var lirrStopsCollection: MongoCollection<BSONDocument> {
    self.application.mongoDB.client.db("mta").collection("lirrStopsData")
  }

  func findFeedMessages() async throws -> [BSONDocument] {
    do {
      return try await self.feedMessageCollection.find().toArray()
    } catch {
      throw Abort(.internalServerError, reason: "Failed to load feed messages: \(error)")
    }
  }
  
  func findStops() async throws -> [BSONDocument] {
    do {
      return try await self.lirrStopsCollection.find().toArray()
    } catch {
      throw Abort(.internalServerError, reason: "Failed to load feed messages: \(error)")
    }
  }

  func feed(ws: WebSocket) async throws {
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
