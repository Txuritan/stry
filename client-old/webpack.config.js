const { readFileSync } = require("fs");
const path = require("path");

const webpack = require("webpack");

const { CleanWebpackPlugin } = require("clean-webpack-plugin");

const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const TerserPlugin = require("terser-webpack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");

const { gitDescribeSync } = require("git-describe");

const info = gitDescribeSync({
    dirtyMark: "-modified"
});

const package = JSON.parse(readFileSync(path.join(__dirname, "package.json")));

module.exports = (env, argv) => {
    const isProd = argv.mode === "production";

    return {
        entry: "./src/index.ts",
        mode: "development",
        devtool: "inline-source-map",
        output: {
            filename: "[name].[contenthash].js",
            path: path.resolve(__dirname, isProd ? "dist" : "temp")
        },
        resolve: {
            extensions: [ ".tsx", ".ts", ".js", ".scss", ".sass" ]
        },
        devServer: {
            host: '0.0.0.0',
            proxy: {
                "/": "http://localhost:8901"
            }
        },
        node: {
            global: false,
            process: false,
            __filename: false,
            __dirname: false,
            Buffer: false,
            setImmediate: false,
            dns: false,
            fs: false,
            path: false,
            url: false
        },
        plugins: [
            new CleanWebpackPlugin(),
            new webpack.ProgressPlugin(),
            new webpack.HashedModuleIdsPlugin(),
            new webpack.DefinePlugin({
                "DEBUG_MODE": !isProd,
                "GIT_VERSION": JSON.stringify(`v${package.version}-${info.suffix}`)
            }),
            new HtmlWebpackPlugin({
                title: "stry",
                template: "./src/index.html",
                inject: false,
                minify: {
                    removeComments: true,
                    collapseWhitespace: true
                }
            }),
            new MiniCssExtractPlugin({
                filename: "[name].[contenthash].css",
            })
        ],
        optimization: {
            namedModules: true,
            runtimeChunk: "single",
            minimizer: [new TerserPlugin()],
            splitChunks: {
                chunks: "all",
                maxInitialRequests: Infinity,
                minSize: 0,
                cacheGroups: {
                    styles: {
                        test: /\.s?css$/,
                        name: "styles",
                        chunks: "all",
                        enforce: true
                    },
                    dateformat: {
                        test: /dateformat/,
                        name: "dateformat"
                    },
                    marked: {
                        test: /marked/,
                        name: "marked"
                    },
                    mithril: {
                        test: /mithril/,
                        name: "mithril"
                    }
                }
            }
        },
        module : {
            rules: [{
                test: /\.tsx?/,
                use: [
                    "babel-loader",
                    "ts-loader"
                ],
                exclude: path.resolve(__dirname, "node_modules")
            }, {
                test: /\.s[ac]ss$/i,
                use: [
                    MiniCssExtractPlugin.loader,
                    {
                        loader: "css-loader",
                        options: {
                            sourceMap: true
                        }
                    },
                    {
                        loader: "sass-loader",
                        options: {
                            sourceMap: true
                        }
                    }
                ]
            }]
        }
    };
};
