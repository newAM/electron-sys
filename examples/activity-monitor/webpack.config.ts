import path from "path";
import webpack from "webpack";
import CopyPlugin from "copy-webpack-plugin";
import WasmPackPlugin from "@wasm-tool/wasm-pack-plugin";

const config: webpack.Configuration = {
  mode: "development",
  /* NOTE: uncommenting the following line (e.g., target: ...) causes the
   * generated output (for the renderer process script) to use node "require".
   * However, "require" is not available when nodeIntegration is disabled, which
   * is now the default behavior of electron for security reasons.
   *
   * In other words, target: "electron-renderer" does not work with the default
   * electron configuration, and in order to make it work, the user must use an
   * insecure configuration for their app. We should not rely on this.
   */
  // target: "electron-renderer",
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
