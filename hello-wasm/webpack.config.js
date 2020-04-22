const path = require('path');
module.exports = {
  entry: "./index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
    publicPath: "./dist/"
  },
  mode: "development",
  devtool: "source-map"
};