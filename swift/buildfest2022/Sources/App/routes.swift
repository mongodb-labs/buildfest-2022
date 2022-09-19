import Vapor

let nealController = NealController()

func routes(_ app: Application) throws {
  app.get { req async in
    "It works!"
  }

  app.get("hello") { req async -> String in
    "Hello, world!"
  }
  
  app.webSocket("feed") { req, ws in
    print(ws)
  }

  try? nealController.boot(routes: app.routes)
}
