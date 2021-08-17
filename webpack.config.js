const path = require("path");
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

const port = process.env.PORT || 3000;

module.exports = {
  mode: "development",
  entry: {
    app: "./js/index.js"
  },
  output: {
    path: dist,
    filename: "[hash].js"
  },
  devtool:"source-map",
  devServer: {
    contentBase: dist,
  },
  resolve: {
		extensions: [".jsx", ".js"]
	},
  experiments:{
    asyncWebAssembly: true
  },
  module: {
		rules: [
      {
        test: /\.(js|jsx)$/,
        exclude: /node_modules/,
        use: {
          // `.swcrc` can be used to configure swc
          loader: 'swc-loader',
        }
      },
      {
        test: /\.(txt|csv)$/,
        use: [
          {
            loader: 'file-loader',
            options: {
              name: "[path][name].[ext]",
              emitFile: true,
            },
          },
        ],
      }
		],
	},
  plugins: [
    new HtmlWebpackPlugin({
      template: './static/index.html',
      filename: 'index.html',
    }),

    new WasmPackPlugin({
      crateDirectory: __dirname,
      forceMode: "development"
    })
  ],
  devServer: {
    host: 'localhost',
    port: port,
    open: false,
  }
};
