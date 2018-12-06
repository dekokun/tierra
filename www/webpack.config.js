const CopyWebpackPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const path = require('path');

module.exports = (env, argv) => ({
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  plugins: [
    new CopyWebpackPlugin(['index.html']),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "../"),
      extraArgs: "--no-typescript",
      forceWatch: argv.mode == "development",
    }),
  ],
});
