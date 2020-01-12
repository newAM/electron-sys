import path from "path";
import webpack from "webpack";
import CopyPlugin from "copy-webpack-plugin";
import WasmPackPlugin from "@wasm-tool/wasm-pack-plugin";

const config: webpack.Configuration = {
  mode: "development",
  entry: "./js/index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
  },
  optimization: {
    minimize: false,
  },
  plugins: [
    new CopyPlugin([
      path.resolve(__dirname, "static")
    ]),
    new WasmPackPlugin({      
      crateDirectory: path.resolve(__dirname, "crates/app-renderer"),
    }),
  ],
};

export default config;
