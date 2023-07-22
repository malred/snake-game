const path = require('path');
const CopyWebpackPlugin = require('copy-webpack-plugin')
module.exports = {
    // 入口
    entry: "./bootstrap.js",
    output: {
        path: path.resolve(__dirname, "public"),
        filename: "bootstrap.js"
    },
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                use: "ts-loader",
                exclude: /node_modules/,
            }
        ]
    },
    resolve: {
        extensions: ['.tsx', '.ts', '.js']
    },
    mode: "development",
    plugins: [
        new CopyWebpackPlugin({
            patterns: [
                // 这里的./是指output的目录
                // index.html => ./public
                { from: "./index.html", to: "./" }
            ]
        })
    ]
}