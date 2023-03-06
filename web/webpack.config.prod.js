const path = require('path')
const webpack = require('webpack')
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')

module.exports = {
  entry: './index.js',
  output: {
    path: path.resolve(__dirname, 'build/prod'),
    filename: 'index.js'
  },
  plugins: [
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, '.'),
      outDir: path.resolve(__dirname, 'build/pkg')
    })
  ],
  mode: 'production',
  experiments: {
    asyncWebAssembly: true
  }
}
