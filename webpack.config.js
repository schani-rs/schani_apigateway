var path = require('path');

module.exports = {
  entry: './client/src/index.js',
  output: {
    filename: 'bundle.js',
    path: path.resolve(__dirname, 'static')
  },
  module: {
    rules: [
      { test: /\.js$/, exclude: /node_modules/, loader: "babel-loader" }
    ]
  }
};
