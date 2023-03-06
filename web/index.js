const rust = import('./build/pkg')
export function render_mandelbrot(context) {
  // Note that a dynamic `import` statement here is required due to
  // webpack/webpack#6615, but in theory `import { foo } from './pkg';`
  // will work here one day as well!
  rust
    .then(m => {
      m.render_mandelbrot(0, BigInt(0), BigInt(0), context)
    })
    .catch(console.error)
}
