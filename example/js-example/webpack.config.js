const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  experiments: {
    asyncWebAssembly: true
  },
  // FIXME: https://github.com/rust-random/getrandom/issues/224 https://github.com/rustwasm/wasm-pack/issues/822
  ignoreWarnings: [
    (warning) =>
      warning.message ===
      "Critical dependency: the request of a dependency is an expression",
  ],
  plugins: [
    new CopyWebpackPlugin({patterns: ['index.html']})
  ],
};
