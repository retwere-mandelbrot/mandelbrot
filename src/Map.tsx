import { CRS, extend, Transformation } from 'leaflet'
import React from 'react'
import { MapContainer, TileLayer } from 'react-leaflet'
import '../node_modules/leaflet/dist/leaflet.css'

type MapProps = {
  height: number
}
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
      // FIXME: this should be the initial viewport:
      bounds={[
        [-2.0, -1.25],
        [0.5, 1.25]
      ]}
      // FIXME: this should be the d limits (no gray area):
      maxBounds={[
        [-4.0, -4.0],
        [4.0, 4.0]
      ]}
    >
      <TileLayer url="./tiles/{z}/{x}/{y}.png" />
    </MapContainer>
  )
}

export default Map
