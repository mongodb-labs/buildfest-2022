import Foundation

struct VehiclePosition: Decodable {
  let position: Position
  let vehicle: VehicleDescriptor
  
  enum CodingKeys: String, CodingKey {
    case position
    case vehicle
  }
}
