import Foundation
import MapKit

struct Position: Decodable {
  let latitude: Double
  let longitude: Double
  
  var coordinate: CLLocationCoordinate2D {
    CLLocationCoordinate2D(latitude: latitude, longitude: longitude)
  }

  enum CodingKeys: String, CodingKey {
    case latitude
    case longitude
  }
}
