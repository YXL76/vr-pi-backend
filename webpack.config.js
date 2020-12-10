const { ESBuildPlugin, ESBuildMinifyPlugin } = require("esbuild-loader");
const { resolve } = require("path");

target = "es2018";

/**@type {import('webpack').Configuration}*/
module.exports = {
  context: resolve(__dirname, "static"),
  stats: {
    assets: false,
    children: false,
    chunkModules: false,
    chunks: false,
    colors: true,
    entrypoints: false,
    modules: false,
  },
  mode: "none",
  performance: {
    hints: false,
  },
  optimization: {
    minimize: true,
    minimizer: [new ESBuildMinifyPlugin({ target })],
  },
  entry: {
    index: resolve(__dirname, "static", "index.ts"),
  },
  output: {
    path: resolve(__dirname, "static"),
    filename: "[name].js",
  },
  module: {
    rules: [
      {
        test: /\.js$/,
        loader: "esbuild-loader",
        options: { target },
      },
      {
        test: /\.ts$/,
        loader: "esbuild-loader",
        options: {
          loader: "ts",
          target,
          tsconfigRaw: require(resolve(__dirname, "tsconfig.json")),
        },
      },
    ],
  },
  plugins: [new ESBuildPlugin()],
};
