import SwiftUI
import MapKit

struct MapView: View {
  @EnvironmentObject var trainTracker: TrainTracker
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
  
  var body: some View {
    Map(coordinateRegion: $region, annotationItems: trainTracker.entities) { entity in
      MapAnnotation(coordinate: entity.vehicle.position.coordinate) {
        Image(systemName: "tram.circle.fill")
          .font(.title)
          .foregroundColor(.black)
          .background(.white)
          .clipShape(Circle())
          .opacity(100)
      }
    }.onAppear {
      trainTracker.start()
    }.edgesIgnoringSafeArea(.all)
  }
}

struct MapView_Previews: PreviewProvider {
  static var previews: some View {
    MapView().environmentObject(TrainTracker())
  }
}
