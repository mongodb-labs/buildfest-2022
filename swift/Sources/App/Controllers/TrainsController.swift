import Vapor
import MongoSwift
import NIO

// Access information on trains

struct TrainsController: RouteCollection {
    func boot(routes: RoutesBuilder) throws {
        let trains = routes.grouped("trains")
        trains.group(":id") { train in
            train.get(use: show)
        }
    }
    
    func show(req: Request) async throws -> String {
        do {
            guard let trainId = req.parameters.get("id") else { return "Endpoint requires a train id" }
            let train = try await req.findTrain(trainId: trainId)
            // TODO: handle Train Optional nil value
                
            let encoder = ExtendedJSONEncoder()
            encoder.format = .canonical
            let trainAsJson = try encoder.encode(train)
            return String(decoding: trainAsJson, as: UTF8.self)
        } catch {
            return "Unable to process request :\(error)"
        }
    }
}

extension Request {
    // May need to change based on how we store the location info
    var trainsCollection: MongoCollection<Train> {
        self.application.mongoDB.client.db("mta").collection("trains", withType: Train.self)
    }
    func findTrain(trainId: String) async throws -> Train? {
        do {
            let query: BSONDocument = ["train_id": BSON.string(trainId)]
            return try await self.trainsCollection.findOne(query)
        } catch {
            throw Abort(.internalServerError, reason: "Query failed: \(error)")
        }
    }
}

