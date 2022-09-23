import Foundation

class StopsRepository: ObservableObject {
  @Published var stops: [String:Stop] = [:]
  
  func get(stopId: String) -> Stop {
    self.stops[stopId] ?? Stop(stopId: "1", stopCode: "MAD", stopName: "Penn Station")
  }
  
  func loadStops() {
    guard let url = URL(string: "http://localhost:8080/stops") else { fatalError("Missing URL") }
    let urlRequest = URLRequest(url: url)
    let dataTask = URLSession.shared.dataTask(with: urlRequest) { (data, response, error) in
      if let error = error {
        print("Request error: ", error)
        return
      }

      guard let response = response as? HTTPURLResponse else { return }

      if response.statusCode == 200 {
        guard let data = data else { return }
        DispatchQueue.main.async {
          do {
            let decodedStops = try JSONDecoder().decode([Stop].self, from: data)
            for decodedStop in decodedStops {
              self.stops[decodedStop.stopId] = decodedStop
            }
          } catch let error {
            print("Error decoding: ", error)
          }
        }
      }
    }

    dataTask.resume()
  }
}
