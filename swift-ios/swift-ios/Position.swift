import Foundation
import MapKit

struct Position: Decodable {
  let latitude: Double
  let longitude: Double
  let bearing: Double
  let speed: Double
  
  var coordinate: CLLocationCoordinate2D {
    CLLocationCoordinate2D(latitude: latitude, longitude: longitude)
  }
}
