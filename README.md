# Retwere's Mandelbrot Project

## Development

Monorepo for Mandelbrot project.

Uses a `cargo` workspace to manage rust subprojects:
* `apps` -- command line applications
* `fractal` -- library for rendering Mandelbrot and Julia sets
* `web` -- wasm wrapper library for `fractal`

Also uses `yarn` to manage JS/TS projects:
* `web` -- wasm wrapper library for `fractal`
* `client` -- react app for rendering fractals in a Leaflet map.
