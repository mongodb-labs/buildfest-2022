import SwiftUI

@main
struct swift_iosApp: App {
  var trainTracker = TrainTracker()
  
  var body: some Scene {
    WindowGroup {
      VStack {
        MapView().environmentObject(trainTracker)
      }
    }
  }
}
