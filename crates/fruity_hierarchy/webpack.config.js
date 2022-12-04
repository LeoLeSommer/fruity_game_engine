import path from 'path';
import webpack from 'webpack';
import WasmPackPlugin from '@wasm-tool/wasm-pack-plugin';
import HtmlWebpackPlugin from 'html-webpack-plugin';
import {fileURLToPath} from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export default {
    entry: './index_browser.js',
    output: {
        path: path.resolve(__dirname, 'dist/browser'),
        filename: 'index_browser.js',
    },
    plugins: [
        new HtmlWebpackPlugin(),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, "."),
            extraArgs: '--features wasm-module',
        }),
        new webpack.ProvidePlugin({
          TextDecoder: ['text-encoding', 'TextDecoder'],
          TextEncoder: ['text-encoding', 'TextEncoder']
        })
    ],
    mode: 'development',
    experiments: {
        asyncWebAssembly: true
    },
    devServer: {
        transportMode: 'ws', 
        injectClient: false,
    }
};