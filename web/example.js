import { render_mandelbrot } from "."

const tile = document.createElement('canvas')
tile.height = 512
tile.width = 512
render_mandelbrot(tile.getContext('2d'))
document.body.appendChild(tile)
