const path = require('path')
const HtmlWebpackPlugin = require('html-webpack-plugin')
const webpack = require('webpack')
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')

module.exports = {
  context: path.resolve(__dirname, '.'),
  entry: path.resolve(__dirname, './example.js'),
  output: {
    path: path.resolve(__dirname, 'build/dev'),
    filename: 'index.js'
  },
  plugins: [
    new HtmlWebpackPlugin(),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, '.'),
      outDir: path.resolve(__dirname, 'build/pkg')
    })
  ],
  devtool: 'inline-source-map',
  mode: 'development',
  experiments: {
    asyncWebAssembly: true
  }
}
