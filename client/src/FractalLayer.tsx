import init, { render_mandelbrot } from 'fractal-web'
import { Coords, DoneCallback } from 'leaflet'
import GridLayer from './GridLayer'

function FractalLayer() {
  function createTile(coords: Coords, done: DoneCallback): HTMLCanvasElement {
    let tile = document.createElement('canvas')
    tile.width = 512
    tile.height = 512
    let context = tile.getContext('2d')!
    init()
      .then(() => {
        render_mandelbrot(coords.z, BigInt(coords.x), BigInt(coords.y), context)
        done(undefined, tile)
      })
      .catch(e => {
        console.error(e)
        done(e)
      })
    return tile
  }

  return <GridLayer createTile={createTile} />
}
export default FractalLayer
