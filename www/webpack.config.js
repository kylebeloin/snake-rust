const path = require("path");
const CopyWebpackPlugin = require("copy-webpack-plugin");

module.exports = {
  entry: "./bootstrap.js",
  output: {
    filename: "bootstrap.js",
    path: path.resolve(__dirname, "public"),
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin({
      patterns: [{ from: "./index.html", to: "./" }],
    }),
  ],
};
