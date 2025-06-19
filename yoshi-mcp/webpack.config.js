/**
 * Webpack configuration for Yoshi MCP VS Code Extension
 */

const path = require('path');

module.exports = {
    target: 'node',
    mode: 'none',
    
    entry: './src/extension.ts',
    
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'extension.js',
        libraryTarget: 'commonjs2'
    },
    
    externals: {
        vscode: 'commonjs vscode'
    },
    
    resolve: {
        extensions: ['.ts', '.js', '.wasm']
    },
    
    module: {
        rules: [
            {
                test: /\.ts$/,
                exclude: /node_modules/,
                use: [
                    {
                        loader: 'ts-loader'
                    }
                ]
            },
            {
                test: /\.wasm$/,
                type: 'asset/resource'
            }
        ]
    },
    
    devtool: 'nosources-source-map',
    
    infrastructureLogging: {
        level: "log"
    },
    
    optimization: {
        minimize: true,
        splitChunks: {
            chunks: 'all',
            cacheGroups: {
                wasm: {
                    test: /\.wasm$/,
                    name: 'wasm-modules',
                    chunks: 'all'
                }
            }
        }
    }
};
