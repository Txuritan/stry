const { writeFileSync, readFileSync } = require("fs");
const path = require("path");
const os = require("os");

const { gitDescribeSync } = require("git-describe");
const { renderSync } = require("sass");

const info = gitDescribeSync({
    dirtyMark: "-modified"
});

const dev = process.env.NODE_ENV !== "prod";

const package = readFileSync(path.join(__dirname, "package.json"));
const packageJson = JSON.parse(package);

const style = renderSync({ file: "./src/scss/index.scss" }).css;
const script = readFileSync(path.join(__dirname, "dist", "index.bundle.js"));

const output = `<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta http-equiv="X-UA-Compatible" content="ie=edge">
    <title>Stry2</title>
    <style>${style}</style>
</head>

<body id="body" pad="body">
    <script>
        var stryVersion = {
            "git": \"${info.suffix}\",
            "package": \"${packageJson.version}\",
        };
        var debug = ${dev};
    </script>
    <script>${script}</script>
</body>

</html>`;

writeFileSync(path.join(__dirname, "dist", "index.html"), output);