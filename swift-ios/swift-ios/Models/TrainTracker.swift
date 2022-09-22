import Foundation

class TrainTracker: ObservableObject {
  @Published var entities: [Entity] = [
    Entity(
      id: "1",
      vehicle: VehiclePosition(
        position: Position(latitude: 40.758896, longitude: -73.985130),
        vehicle: VehicleDescriptor(id: "1", label: "LIRR")
      )
    )
  ]
  
  let task = URLSession.shared.webSocketTask(with: URL(string: "ws://localhost:8080/feed")!)

  public func start() {
    task.resume()
    getSchedule()
  }
  
  public func getSchedule() {
    task.receive { result in
      switch result {
      case .failure(let error):
        print(error)
      case .success(let message):
        switch message {
        case .string(let text):
          let decoder = JSONDecoder()
          do {
            let decoded = try decoder.decode(Message.self, from: Data(text.utf8))
            DispatchQueue.main.async {
              self.entities = decoded.entities
            }
            self.getSchedule()
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
