import Vapor
import MongoSwift
import NIO

struct NealController: RouteCollection {
    func boot(routes: RoutesBuilder) throws {
        let neal = routes.grouped("neal")
        neal.get(use: index)
        neal.post(use: create)

        neal.group(":id") { todo in
            todo.get(use: show)
            todo.put(use: update)
            todo.delete(use: delete)
        }
    }

    func index(req: Request) async throws -> String {

        return "ok"
    }

    func create(req: Request) async throws -> String {
        // ...
        return "created neal"
    }

    func show(req: Request) async throws -> String {
        guard let id = req.parameters.get("id") else {
            throw Abort(.internalServerError)
        }
        return "showing \(id)"
    }

    func update(req: Request) async throws -> String {
        guard let id = req.parameters.get("id") else {
            throw Abort(.internalServerError)
        }
        return "updating \(id)"
    }

    func delete(req: Request) async throws -> String {
        guard let id = req.parameters.get("id") else {
            throw Abort(.internalServerError)
        }
        return "deleting \(id)"
    }
}
