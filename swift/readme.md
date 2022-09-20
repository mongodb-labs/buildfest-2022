# Swifty MTA Data

### Tuesday Ideation
- Durran: iOS app with a map and subway icons
  - websocket on iOS
- Chris: Working on Vapor controllers
  - CS Pipeline controlled from front end
- Neal: Working on browser app
  - Make MTA script pull http proto

#### Bugs

Our websocket is not emitting things from the change stream, are we watching correctly?

`TrainsController.swift`
- BSONDocument did not accept string directly, need to use BSON.string
- Port in use ghost, weird...
