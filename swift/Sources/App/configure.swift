import Vapor
import MongoDBVapor

// configures your application
public func configure(_ app: Application) throws {
  // uncomment to serve files from /Public folder
  try app.mongoDB.configure(ProcessInfo.processInfo.environment["MONGODB_URI"] ?? "mongodb://localhost:27018/mta?replicaSet=replset")
  // app.middleware.use(FileMiddleware(publicDirectory: app.directory.publicDirectory))
  ContentConfiguration.global.use(encoder: ExtendedJSONEncoder(), for: .json)
  ContentConfiguration.global.use(decoder: ExtendedJSONDecoder(), for: .json)
  // register routes
  try routes(app)
}
