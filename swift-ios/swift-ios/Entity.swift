import Foundation
import SwiftUI

struct Entity: Decodable, Identifiable {
  let id: String
  let vehicle: VehiclePosition
  
  var name: String {
    vehicle.vehicle.label
  }
  
  var color: Color {
    switch vehicle.vehicle.label {
      case "1", "2", "3": return Color.red
      case "4", "5", "6": return Color.green
      case "Q", "N", "R": return Color.yellow
      case "E", "F", "B": return Color.orange
      default: return Color.blue
    }
  }
}
