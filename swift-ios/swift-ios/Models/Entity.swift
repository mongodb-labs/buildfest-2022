import Foundation
import MapKit
import SwiftUI

struct Entity: Decodable, Identifiable {
  let id: String
  let vehicle: VehiclePosition
  
  var name: String {
    vehicle.vehicle.label
  }
  
  var coordinate: CLLocationCoordinate2D {
    vehicle.position.coordinate
  }
  
  func route(stop: Stop) -> String {
    "\(vehicle.vehicle.label) \(vehicle.currentStatus) \(stop.stopName)"
  }
  
  enum CodingKeys: String, CodingKey {
    case id
    case vehicle
  }
}
