import SwiftUI

@main
struct EmTeeAy: App {
  var trainsRepository = TrainsRepository()
  var stopsRepository = StopsRepository()
  
  var body: some Scene {
    WindowGroup {
      VStack {
        MapView().environmentObject(trainsRepository).environmentObject(stopsRepository)
      }
    }
  }
}
