import { Transformation, Icon, Projection, CRS, extend, LatLngExpression } from 'leaflet'
import { MapContainer, TileLayer, Marker, Popup } from 'react-leaflet'
import '../node_modules/leaflet/dist/leaflet.css'

type MapProps = {
  height: number
}

/* CRS
Methods
Method	Returns	Description
latLngToPoint(<LatLng> latlng, <Number> zoom)	Point
Projects geographical coordinates into pixel coordinates for a given zoom.

pointToLatLng(<Point> point, <Number> zoom)	LatLng
The inverse of latLngToPoint. Projects pixel coordinates on a given zoom into geographical coordinates.

project(<LatLng> latlng)	Point
Projects geographical coordinates into coordinates in units accepted for this CRS (e.g. meters for EPSG:3857, for passing it to WMS services).

unproject(<Point> point)	LatLng
Given a projected coordinate returns the corresponding LatLng. The inverse of project.

scale(<Number> zoom)	Number
Returns the scale used when transforming projected coordinates into pixel coordinates for a particular zoom. For example, it returns 256 * 2^zoom for Mercator-based CRS.

zoom(<Number> scale)	Number
Inverse of scale(), returns the zoom level corresponding to a scale factor of scale.

getProjectedBounds(<Number> zoom)	Bounds
Returns the projection's bounds scaled and transformed for the provided zoom.

distance(<LatLng> latlng1, <LatLng> latlng2)	Number
Returns the distance between two geographical coordinates.

wrapLatLng(<LatLng> latlng)	LatLng
Returns a LatLng where lat and lng has been wrapped according to the CRS's wrapLat and wrapLng properties, if they are outside the CRS's bounds.

wrapLatLngBounds(<LatLngBounds> bounds)	LatLngBounds
Returns a LatLngBounds with the same size as the given one, ensuring that its center is within the CRS's bounds. Only accepts actual L.LatLngBounds instances, not arrays.

Properties
Property	Type	Description
code	String	Standard code name of the CRS passed into WMS services (e.g. 'EPSG:3857')
wrapLng	Number[]	An array of two numbers defining whether the longitude (horizontal) coordinate axis wraps around a given range and how. Defaults to [-180, 180] in most geographical CRSs. If undefined, the longitude axis does not wrap around.
wrapLat	Number[]	Like wrapLng, but for the latitude (vertical) axis.
infinite	Boolean
 */
const mandelbrotCRS = extend({}, CRS.Simple, {
  transformation: new Transformation(32, 128, -32, 128)
})

function Map({ height }: MapProps) {
  return (
    <MapContainer
      center={[0.0, 0.0]}
      zoom={0}
      scrollWheelZoom={false}
      style={{ height }}
      crs={mandelbrotCRS}
      bounds={[
        [-2.0, -1.25],
        [0.5, 1.25]
      ]}
      maxBounds={[
        [-4.0, -4.0],
        [4.0, 4.0]
      ]}
    >
      <TileLayer url="./tiles/{z}/{x}/{y}.png" />
      <Marker position={[0.0, 0.0]} icon={new Icon({ iconUrl: "./marker.png", iconAnchor: [25, 82] })}>
        <Popup>
          <b>0.0, 0.0</b>
        </Popup>
      </Marker>
      <Marker position={[-4.0, -4.0]} icon={new Icon({ iconUrl: "./marker.png", iconAnchor: [25, 82] })}>
        <Popup>
          <b>-4.0, -4.0</b>
        </Popup>
      </Marker>
      <Marker position={[4.0, 4.0]} icon={new Icon({ iconUrl: "./marker.png", iconAnchor: [25, 82] })}>
        <Popup>
          <b>4.0, 4.0</b>
        </Popup>
      </Marker>
    </MapContainer>
  )
}

export default Map
