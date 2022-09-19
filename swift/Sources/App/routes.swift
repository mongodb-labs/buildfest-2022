import Vapor
import MongoDBVapor

let nealController = NealController()

func routes(_ app: Application) throws {
  app.get { req async in
    "It works!"
  }

  app.get("feed-messages") { req async throws -> [FeedMessage] in
    try await req.findFeedMessages()
  }

  app.webSocket("feed") { req, ws async in
    await req.feed(ws: ws)
  }

  try? nealController.boot(routes: app.routes)
}

extension Request {

  var feedMessageCollection: MongoCollection<FeedMessage> {
    self.application.mongoDB.client.db("mta").collection("feedMessages", withType: FeedMessage.self)
  }

  func findFeedMessages() async throws -> [FeedMessage] {
    do {
      return try await self.feedMessageCollection.find().toArray()
    } catch {
      throw Abort(.internalServerError, reason: "Failed to load feed messages: \(error)")
    }
  }
  
  func feed(ws: WebSocket) async {
    let changeStreamTask = Task {
      let encoder = ExtendedJSONEncoder()
      for try await event in try await feedMessageCollection.watch() {
        if let data = try? encoder.encode(event.fullDocument) {
          try await ws.send(String(decoding: data, as: UTF8.self))
        }
      }
    }
    ws.onClose.whenComplete { result in
      changeStreamTask.cancel();
    }
  }
}
