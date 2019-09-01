const { readFileSync } = require("fs");
const path = require("path");

const webpack = require("webpack");

const MiniCssExtractPlugin = require("mini-css-extract-plugin");

const HtmlWebpackPlugin = require("html-webpack-plugin");
const HtmlWebpackInlineSourcePlugin = require("html-webpack-inline-source-plugin");

const { gitDescribeSync } = require("git-describe");

const info = gitDescribeSync({
    dirtyMark: "-modified"
});

const package = JSON.parse(readFileSync(path.join(__dirname, "package.json")));

const isProd = process.env.NODE_ENV === "production";

const config = {
    entry: "./client/src/index.ts",
    mode: "development",
    devtool: "inline-source-map",
    output: {
        filename: "client.bundle.js",
        path: path.resolve(__dirname, "client", "dist"),
    },
    resolve: {
        extensions: [ ".tsx", ".ts", ".js", ".scss", ".sass" ],
    },
    devServer: {
        proxy: {
            "/": "http://localhost:8901"
        },
    },
    plugins: [
        new webpack.DefinePlugin({
            "GIT_VERSION": JSON.stringify(`v${package.version}-${info.suffix}`),
        }),
        new HtmlWebpackPlugin({
            title: "stry",
            template: "./client/src/index.html",
            inject: true,
            minify: {
                removeComments: true,
                collapseWhitespace: false,
            },
        }),
    ],
    module : {
        rules: [{
            test: /\.tsx?/,
            use: [
                "babel-loader",
                "ts-loader",
            ],
            exclude: path.resolve(__dirname, "node_modules"),
        }, {
            test: /\.s[ac]ss$/i,
            use: [
                isProd ? MiniCssExtractPlugin.loader : "style-loader",
                {
                    loader: "css-loader",
                    options: {
                        sourceMap: true,
                    },
                },
                {
                    loader: "sass-loader",
                    options: {
                        sourceMap: true,
                    },
                },
            ],
        }]
    },
};

if (isProd) {
    config.plugins.push(new HtmlWebpackInlineSourcePlugin());
    config.plugins.push(new MiniCssExtractPlugin());
}

module.exports = config;
