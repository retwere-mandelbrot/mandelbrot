# Applications

### fractal
renders mandelbrot & julia set images.
```
cargo run -r --bin fractal -- -o mandelbrot.png --x-res 512 --y-res 512 --x-min -2.0 --x-max 0.5 --y-min -1.25 --y-max 1.25
cargo run -r --bin fractal -- -j -o julia.png --x-res 512 --y-res 512 --x-min -2.0 --x-max 2.0 --y-min -2.0 --y-max 2.0 --cx -0.4 --cy 0.6
```

### tileserver
renders and serves mandelbrot & julia tiles.
```
cargo run -r --bin tileserver
```

Tiles are available at:
```
http://localhost:8000/mandelbrot/{zoom}/{x}/{y}
http://localhost:8000/julia/-0.4/0.6/{zoom}/{x}/{y}
```

## Build
dev build:
```
cargo build
```

release build:
```
cargo build -r
```
