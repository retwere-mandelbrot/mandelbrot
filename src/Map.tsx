import { CRS, extend, Transformation } from 'leaflet'
import { PropsWithChildren } from 'react'
import { MapContainer, TileLayer } from 'react-leaflet'
import '../node_modules/leaflet/dist/leaflet.css'

type FractalProps = {
  height: number,
  url: string
}

const FRACTAL_CRS = extend({}, CRS.Simple, {
  transformation: new Transformation(32, 128, -32, 128)
})

function Fractal({ height, url, children }: PropsWithChildren<FractalProps>) {
  const url_pattern = `${url}/{z}/{x}/{y}/`
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
      <TileLayer url={url_pattern} maxZoom={50} maxNativeZoom={50} />
    </MapContainer>
  )
}

export default Fractal
