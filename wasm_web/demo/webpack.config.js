const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./src/bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin({
      patterns: [{
        from: 'src/index.html'
      }]
    })
  ],
  resolve: {
    extensions: [".html",".wasm",".js",".css"],
    alias: {
     "@": path.resolve(__dirname, "src")
    }
  },
  module: {
    rules: [
      {
        test: /\.css$/,
        use: [
          {
            loader: "css-loader",
            options: {
              modules: true,
            }
          }
        ],
        include: path.resolve(__dirname, 'src')
      }
    ]
  },

  experiments: {
    asyncWebAssembly: true
  }
};
