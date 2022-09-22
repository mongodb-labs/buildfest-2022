import SwiftUI

@main
struct swift_iosApp: App {
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
