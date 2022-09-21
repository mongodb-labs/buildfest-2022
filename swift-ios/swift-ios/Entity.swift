import Foundation
import SwiftUI

struct Entity: Decodable, Identifiable {
  let id: String
  let vehicle: VehiclePosition
  
  var name: String {
    vehicle.vehicle.label
  }
}
