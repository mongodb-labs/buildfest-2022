import Vapor

let nealController = NealController()

func routes(_ app: Application) throws {
    app.get { req async in
        "It works!"
    }

    app.get("hello") { req async -> String in
        "Hello, world!"
    }

    try? nealController.boot(routes: app.routes)
}
