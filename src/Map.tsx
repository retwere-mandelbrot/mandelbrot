import { LatLngExpression } from 'leaflet'
import { MapContainer, TileLayer, Marker, Popup } from 'react-leaflet'
import '../node_modules/leaflet/dist/leaflet.css'

type MapProps = {
  position: LatLngExpression,
  height: number
}

function Map({ position, height }: MapProps) {
  return (
    <MapContainer center={position} zoom={13} scrollWheelZoom={false} style={{ height }}>
      <TileLayer
        attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
        url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
      />
      <Marker position={position}>
        <Popup>
          A pretty CSS3 popup. <br /> Easily customizable.
        </Popup>
      </Marker>
    </MapContainer>
  )
}

export default Map
