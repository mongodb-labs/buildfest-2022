import SwiftUI
import MapKit

struct MapView: View {
  
  @State private var region = MKCoordinateRegion(
    center: CLLocationCoordinate2D(
      latitude: 40.758896,
      longitude: -73.985130
    ),
    span: MKCoordinateSpan(
      latitudeDelta: 0.03,
      longitudeDelta: 0.03
    )
  )
  
  @State private var entities: [Entity] = []
  
  var body: some View {
    Map(coordinateRegion: $region, annotationItems: entities) { entity in
      MapAnnotation(coordinate: entity.vehicle.position.coordinate) {
        Image(systemName: "tram.circle.fill")
          .font(.title)
          .foregroundColor(.blue)
          .symbolRenderingMode(.hierarchical)
          .opacity(100)
      }
    }.task {
      connectWebSocket()
    }.edgesIgnoringSafeArea(.all)
  }
  
  let task = URLSession.shared.webSocketTask(with: URL(string: "ws://localhost:8080/feed")!)
  
  private func connectWebSocket() {
    task.resume()
    listenOnWebSocket()
  }
  
  private func listenOnWebSocket() {
    task.receive { result in
      switch result {
      case .failure(let error):
        print(error)
      case .success(let message):
        switch message {
        case .string(let text):
          let decoder = JSONDecoder()
          do {
            print("JSON received")
            let decoded = try decoder.decode(Message.self, from: Data(text.utf8))
            entities = decoded.entities
            listenOnWebSocket()
          } catch {
            print("Unexpected error: \(error).")
          }
        case .data(let data):
          print(data)
        @unknown default:
          print("Received unknown")
        }
      }
    }
  }
}

struct MapView_Previews: PreviewProvider {
  static var previews: some View {
    MapView()
  }
}
