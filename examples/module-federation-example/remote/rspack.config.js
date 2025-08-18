const rspack = require('@rspack/core');
const { ModuleFederationPlugin } = rspack.container;
const isProd = process.env.NODE_ENV === 'production';

module.exports = {
  mode: 'development',
  entry: './src/index.js',
  target: 'async-node',
  devtool: false,
  output: {
    publicPath: 'http://localhost:3002/',
    library: {
      type: 'commonjs-module',
    },
  },
  devServer: {
    port: 3002,
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
    sideEffects: false,
    usedExports: false,
    providedExports: false,
    concatenateModules: false,
    mangleExports: false,
    nodeEnv: false,
    mergeDuplicateChunks: false,
    removeAvailableModules: false,
    removeEmptyChunks: false,
    splitChunks: false
  },
  plugins: [
    new ModuleFederationPlugin({
      name: 'remote',
      filename: 'remoteEntry.js',
      library: { type: 'commonjs-module' },
      exposes: {
        './Button': './src/Button',
        './utils': './src/utils',
        './dateUtils': './src/dateUtils',
        './functionalUtils': './src/functionalUtils',
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
        'ramda': {
          singleton: true,
          strictVersion: true,
          requiredVersion: '^0.31.3',
          eager: false,
        },
      },
    }),

  ],
};