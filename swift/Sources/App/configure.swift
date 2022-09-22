import Vapor
import MongoDBVapor

// configures your application
public func configure(_ app: Application) throws {

    guard let uri: String = Environment.get("MONGODB_URI") else {
        throw Abort(.internalServerError, reason: "MONGODB_URI required in your Vapor .env file")
    }
    try app.mongoDB.configure(uri)

    // uncomment to serve files from /Public folder
    // app.middleware.use(FileMiddleware(publicDirectory: app.directory.publicDirectory))

    ContentConfiguration.global.use(encoder: ExtendedJSONEncoder(), for: .json)
    ContentConfiguration.global.use(decoder: ExtendedJSONDecoder(), for: .json)

    // register routes
    try routes(app)
}
