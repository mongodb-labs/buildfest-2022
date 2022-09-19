//
//  TrainController.swift
//  
//
//  Created by Christopher Cho on 9/19/22.
//

import Vapor
import MongoSwift
import NIO

// Access information on trains

struct TrainsController: RouteCollection {
    func boot(routes: RoutesBuilder) throws {
        let trains = routes.grouped("trains")
        
        //trains.get(use: index)
        //trains.group(":id") { train in
        //    trains.get(use: show)
        //}

    }
    /*
    func index(req: Request) async throws -> String {
        // TODO:
    }

    func show(req: Request) async throws -> String {
        // TODO:
    }
    */
}

extension Request {
    // May need to change based on how we store the location info
    var trainsCollection: MongoCollection<Train> {
        self.application.mongoDB.client.db("mta").collection("trains", withType: Train.self)
    }
    func findTrain(trainId: String) async throws -> [Train] {
        do {
            //TODO: how do I make a query? Currently erroring becaues it expects BSON, not a String
            //let query: BSONDocument = ["train_id": trainId]
            return try await self.trainsCollection.find().toArray()
        } catch {
            throw Abort(.internalServerError, reason: "Failed to find train (id \(trainId)): \(error)")
        }
    }
}

