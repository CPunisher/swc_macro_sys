const rspack = require('@rspack/core');
const { ModuleFederationPlugin } = rspack.container;
const isProd = process.env.NODE_ENV === 'production';

module.exports = {
  mode: 'development',
  devtool: false,
  entry: './src/index.js',
  target: 'async-node',
  devtool: false,
  output: {
    publicPath: 'http://localhost:3001/',
    library: {
      type: 'commonjs-module',
    },
  },
  devServer: {
    port: 3001,
  },
  module: {
    rules: [
      {
        test: /\.jsx?$/,
        exclude: /node_modules/,
        use: {
          loader: 'builtin:swc-loader',
          options: {
            jsc: {
              parser: {
                syntax: 'ecmascript',
                jsx: true,
              },
              target: 'es2015',
            },
          },
        },
      },
    ],
  },
  optimization: {
    minimize: false,
  },
  plugins: [
    new ModuleFederationPlugin({
      name: 'host',
      remoteType: 'commonjs-module',
      remotes: {
        remote: '../remote/dist/remoteEntry.js',
      },
      shared: {
        react: {
          singleton: true,
          requiredVersion: '^18.3.1',
          eager: false,
        },
        'react-dom': {
          singleton: true,
          requiredVersion: '^18.3.1',
          eager: false,
        },
        'lodash-es': {
          singleton: true,
          strictVersion: true,
          requiredVersion: '^4.17.21',
          eager: false,
        },
        'date-fns': {
          singleton: true,
          strictVersion: true,
          requiredVersion: '^4.1.0',
          eager: false,
        },
        ramda: {
          singleton: true,
          strictVersion: true,
          requiredVersion: '^0.31.3',
          eager: false,
        },
      },
    }),

  ],
};