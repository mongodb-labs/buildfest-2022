import Foundation

struct VehiclePosition: Decodable {
  let position: Position
  let vehicle: VehicleDescriptor
  let currentStatus: String
  let stopId: String
  
  enum CodingKeys: String, CodingKey {
    case position
    case vehicle
    case currentStatus
    case stopId
  }
}
