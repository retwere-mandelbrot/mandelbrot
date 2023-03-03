// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { foo } from './pkg';`
// will work here one day as well!
const rust = import('./pkg')

rust
  .then(m => {
    let tile = document.createElement('canvas')
    document.body.appendChild(tile)
    //m.render_mandelbrot(0, BigInt(0), BigInt(0), tile.getContext('2d'))
    m.render_smiley(tile.getContext('2d'))
  })
  .catch(console.error)
