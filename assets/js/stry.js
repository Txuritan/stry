function h(tag, attribute, child) {
    var element = document.createElement(tag);

    if (attribute instanceof Array) {
        for (var i = 0; i < attribute.length; i++) {
            if (attribute[i] instanceof Array) {
                element.setAttribute(attribute[i][0], attribute[i][1]);
            } else if (attribute[i] instanceof String || typeof attribute[i] === "string") {
                element.setAttribute(attribute[i][0], "");
            }
        }
    }

    if (child instanceof Array) {
        for (var i = 0; i < child.length; i++) {
            if (child[i] instanceof Node) {
                element.appendChild(child[i]);
            } else {
                element.appendChild(document.createTextNode(child[i]));
            }
        }
    }

    return element;
}

var toaster = {
    received: {
        "aaaaaa": {
            title: "download finished",
            body: "[Harry and Harley](/story/GczMyx/1) by [Rihaan](/author/tc5YzD) has finished downloading",
        }
    },
    visible: [],
    newToast: function (id, title, body) {
        return h("div", [["id", "toast-" + id], "toast", ["l-bg", "gray-100"], ["d-bg", "black-700"], "shadow"], [
            h("div", ["head", "flex"], [
                h("p", [["flex-item", "grow"]], [title]),
                h("p", null, [
                    h("button", null, ["&#10005;"])
                ]),
            ]),
            h("div", ["body"], [
                window.markdownit().render(body),
            ]),
        ]);
    },
};
