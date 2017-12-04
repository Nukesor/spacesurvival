var path = require('path');
var webpack = require('webpack');

// entry and output path/filename variables
const entryPath = path.join(__dirname, 'client/entry.js');
const outputPath = path.join(__dirname, 'static');
const outputFilename = 'main.js';

module.exports = {
    output: {
        path: outputPath,
        filename: `${outputFilename}`,
        publicPath: 'http://localhost:8080/static/'
    },
    resolve: {
        extensions: ['.js', '.elm'],
        modules: ['node_modules']
    },
    module: {
        noParse: /\.elm$/,
        rules: [{
            test: /\.(eot|ttf|woff|woff2|svg)$/,
            use: 'file-loader?publicPath=../../&name=static/css/[hash].[ext]'
        }, {
            test: /\.elm$/,
            exclude: [/elm-stuff/, /node_modules/],
            use: [{
                    loader: 'elm-hot-loader'
                }, {
                    loader: 'elm-webpack-loader',
                    options: {
                        verbose: true,
                        warn: true,
                        debug: true,
                        maxInstances: 4
                    }
            }]
        }]
    },
    entry: [
        'webpack-dev-server/client?http://localhost:8080',
        entryPath
    ],
    devServer: {
        contentBase: './assets',
        hot: true,
        headers: { 'Access-Control-Allow-Origin': '*' }
    }
};
