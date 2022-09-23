import SwiftUI
import MapKit

struct MapAnnotationView: View {
  @State private var showTitle = false
  
  let tooltip: String;
  
  var body: some View {
    VStack() {
      Text(tooltip)
        .font(Font.caption)
        .padding(5)
        .background(Color.white)
        .cornerRadius(10)
        .fixedSize(horizontal: true, vertical: false)
        .opacity(showTitle ? 1 : 0)
      Image(systemName: "tram.circle.fill")
        .font(.title)
        .foregroundColor(.black)
        .background(.white)
        .clipShape(Circle())
        .opacity(100)
    }.onTapGesture {
      withAnimation(.easeInOut) {
        showTitle.toggle()
      }
    }
  }
}

struct MapAnnotationView_Previews: PreviewProvider {
  let entity = Entity(
    id: "1",
    vehicle: VehiclePosition(
      position: Position(latitude: 40.758896, longitude: -73.985130),
      vehicle: VehicleDescriptor(id: "1", label: "204"),
      currentStatus: "IN TRANSIT TO",
      stopId: "231"
    )
  )
  
  static var previews: some View {
    MapAnnotationView(tooltip: "Test")
  }
}
