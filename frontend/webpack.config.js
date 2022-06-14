const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

const srcDir = path.resolve(__dirname, 'src');
const distDir = path.resolve(__dirname, 'dist');

module.exports = {
    mode: 'development',
    entry: path.resolve(srcDir, 'index.js'),
    output: {
        path: distDir,
        filename: 'main.js'
    },
    plugins: [new HtmlWebpackPlugin({
        template: path.resolve(srcDir, 'index.html')
    })],

    devServer: {
        static: {
            directory: distDir
        }
    }
};
