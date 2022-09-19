import Foundation
import Vapor

struct Position: Content {
  let latitude: Float
  let longitude: Float
  let bearing: Float
  let odometer: Double
  let speed: Float
}
