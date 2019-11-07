var light = document.getElementById("mode-light");
var dark = document.getElementById("mode-dark");

function setMode(mode) {
    if (mode) {
        document.body.setAttribute("theme", "dark");

        light.setAttribute("nav-item", "");
        dark.setAttribute("nav-item", "brand");

        Cookies.set("mode", "dark");
    } else {
        document.body.setAttribute("theme", "light");

        light.setAttribute("nav-item", "brand");
        dark.setAttribute("nav-item", "");

        Cookies.set("mode", "light");
    }
}

light.addEventListener("click", function (e) {
    e.preventDefault();

    setMode(false);
});

dark.addEventListener("click", function (e) {
    e.preventDefault();

    setMode(true);
});

setMode(Cookies.get("mode") !== "light");