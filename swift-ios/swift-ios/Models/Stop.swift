import Foundation

struct Stop: Decodable {
  let stopId: String
  let stopCode: String
  let stopName: String
  
  enum CodingKeys: String, CodingKey {
    case stopId = "stop_id"
    case stopCode = "stop_code"
    case stopName = "stop_name"
  }
}
