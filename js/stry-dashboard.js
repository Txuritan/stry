var elmAddUrls = document.getElementById("js__elm--add-urls");

if (elmAddUrls) {
    document
        .getElementById("js__btn-open--add-urls")
        .addEventListener("click", function () {
            if (elmAddUrls.hasAttribute("hidden")) {
                elmAddUrls.removeAttribute("hidden");
            }
        });

    document
        .getElementById("js__btn-close--add-urls")
        .addEventListener("click", function () {
            if (!elmAddUrls.hasAttribute("hidden")) {
                elmAddUrls.setAttribute("hidden", "");
            }
        });
}
