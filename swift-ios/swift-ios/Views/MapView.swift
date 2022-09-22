import SwiftUI
import MapKit

struct MapView: View {
  @EnvironmentObject var trainsRepository: TrainsRepository
  @EnvironmentObject var stopsRepository: StopsRepository
  
  @State var region = MKCoordinateRegion(
    center: CLLocationCoordinate2D(
      latitude: 40.758896,
      longitude: -73.985130
    ),
    span: MKCoordinateSpan(
      latitudeDelta: 0.03,
      longitudeDelta: 0.03
    )
  )
  @State private var showTitle = false
  
  var body: some View {
    Map(coordinateRegion: $region, annotationItems: trainsRepository.entities) { entity in
      MapAnnotation(coordinate: entity.coordinate) {
        let toolTip = entity.route(stop: stopsRepository.get(stopId: entity.vehicle.stopId))
        MapAnnotationView(tooltip: toolTip)
      }
    }.onAppear {
      stopsRepository.loadStops()
      trainsRepository.refreshTrains()
    }.edgesIgnoringSafeArea(.all)
  }
}

struct MapView_Previews: PreviewProvider {
  static var previews: some View {
    MapView().environmentObject(TrainsRepository()).environmentObject(StopsRepository())
  }
}
