import { CRS, extend, Transformation } from 'leaflet'
import { PropsWithChildren } from 'react'
import { MapContainer } from 'react-leaflet'
import '../node_modules/leaflet/dist/leaflet.css'
import FractalLayer from './FractalLayer'

type MapProps = {
  height: number,
  url: string
}

export const FRACTAL_CRS = extend({}, CRS.Simple, {
  transformation: new Transformation(32, 128, -32, 128)
})

function Map({ height, url, children }: PropsWithChildren<MapProps>) {
  return (
    <MapContainer
      center={[0.0, 0.0]}
      zoom={3}
      minZoom={0}
      maxZoom={50}
      scrollWheelZoom={false}
      style={{ height }}
      crs={FRACTAL_CRS}
    >
      {children}
      <FractalLayer />
    </MapContainer>
  )
}

export default Map
