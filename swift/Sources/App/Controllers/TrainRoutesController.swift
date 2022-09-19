import Vapor
import MongoSwift
import NIO

// Track either a train by id or all trains on a train route

struct TrainRoutesController: RouteCollection {
    func boot(routes: RoutesBuilder) throws {
        let trainRoutes = routes.grouped("train_routes")
        
        trainRoutes.get(use: index)
        trainRoutes.post(use: create)
        trainRoutes.group(":id") { train in
            trainRoutes.get(use: show)
            trainRoutes.put(use: update)
        }
    }
    
    /*
    func index(req: Request) async throws -> String {
        // TODO:
    }
    func create(req: Request) throws -> EventLoopFuture<String> {
        // TODO:
    }
    func show(req: Request) async throws -> String {
        // TODO:
    }
    func update(req: Request) throws -> EventLoopFuture<String> {
        // TODO:
    } */
}
extension Request {

    var trainRoutesCollection: MongoCollection<TrainRoute> {
        self.application.mongoDB.client.db("mta").collection("trains", withType: TrainRoute.self)
    }
    func findTrainRoute(trainId: String) async throws -> TrainRoute {

        // TODO: how do I do this?
        //let query: BSONDocument = ["train_id": trainId]
        //return self.trainRoutesCollection.find(query).first
        return TrainRoute(id: "abc", description: "def")

    }
    
    func findTrainRoputes() async throws -> [TrainRoute] {
        return try await self.trainRoutesCollection.find().toArray()
    }
}

