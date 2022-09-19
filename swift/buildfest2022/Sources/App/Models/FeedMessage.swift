import Foundation
import Vapor
import MongoSwift

struct FeedMessage: Content {
  let _id: BSONObjectID
  let feedHeader: FeedHeader
  let feedEntity: FeedEntity
}
