import Foundation
import Vapor

struct FeedMessage: Content {
  let feedHeader: FeedHeader
  let feedEntity: FeedEntity
}
