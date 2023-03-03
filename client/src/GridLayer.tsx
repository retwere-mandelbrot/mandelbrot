import { useLeafletContext } from '@react-leaflet/core'
import { Coords, DoneCallback, GridLayer as L_GridLayer } from 'leaflet'
import { useEffect } from 'react'

type GridLayerProps = {
  createTile: (coords: Coords, done: DoneCallback) => HTMLElement
}

function GridLayer({ createTile }: GridLayerProps) {
  const context = useLeafletContext()

  useEffect(() => {
    const Grid = L_GridLayer.extend({ createTile })
    const map = context.map
    const layer = new Grid()
    map.addLayer(layer)
    return () => {
      map.removeLayer(layer)
    }
  })

  return null
}
export default GridLayer
