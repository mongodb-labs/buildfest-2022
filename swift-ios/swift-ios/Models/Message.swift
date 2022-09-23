import Foundation

struct Message: Decodable {
  let entities: [Entity]
  
  enum CodingKeys: String, CodingKey {
    case entities
  }
}
