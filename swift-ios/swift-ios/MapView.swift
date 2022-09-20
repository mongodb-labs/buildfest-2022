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
  
  @State private var entities = [
    Entity(
      id: "1",
      vehicle: VehiclePosition(
        position: Position(latitude: 40.758896, longitude: -73.975130),
        vehicle: VehicleDescriptor(id: "F", label: "F")
      )
    ),
    Entity(
      id: "2",
      vehicle: VehiclePosition(
        position: Position(latitude: 40.758896, longitude: -73.985130),
        vehicle: VehicleDescriptor(id: "1", label: "1")
      )
    ),
    Entity(
      id: "3",
      vehicle: VehiclePosition(
        position: Position(latitude: 40.758896, longitude: -73.977130),
        vehicle: VehicleDescriptor(id: "Q", label: "Q")
      )
    ),
  ]
  
  var body: some View {
    Map(coordinateRegion: $region, annotationItems: entities) { entity in
      MapAnnotation(coordinate: entity.vehicle.position.coordinate) {
        Text(entity.name)
          .font(.system(size: 15, weight: .bold))
          .foregroundColor(.white)
          .padding(5)
          .background(entity.color)
          .clipShape(Circle())
      }
    }.task {

    }.edgesIgnoringSafeArea(.all)
  }
}

struct MapView_Previews: PreviewProvider {
  static var previews: some View {
    MapView()
  }
}
