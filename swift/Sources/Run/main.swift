import App
import Vapor
import MongoDBVapor

var env = try Environment.detect()
try LoggingSystem.bootstrap(from: &env)
let app = Application(env)
defer { app.shutdown() }
try configure(app)
try app.run()

defer {
  // Cleanup the application's MongoDB data.
  app.mongoDB.cleanup()
  // Clean up the driver's global state. The driver will no longer be usable from this program after this method is
  // called.
  cleanupMongoSwift()
}
