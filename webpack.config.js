const path = require('path')
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')

module.exports = {
  entry: './polars/index', // input file of the JS bundle
  output: {
    // filename: 'bundle.js', // output filename
    path: path.resolve(__dirname, 'dist'), // directory of where the bundle will be created at
    // le
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
    ],
      
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js'],
  },

}