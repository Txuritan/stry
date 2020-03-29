function h(tag, attribute, child) {
    const element = document.createElement(tag);

    if (attribute instanceof Array) {
        for (const i = 0; i < attribute.length; i++) {
            if (attribute[i] instanceof Array) {
                element.setAttribute(attribute[i][0], attribute[i][1]);
            } else if (attribute[i] instanceof String || typeof attribute[i] === "string") {
                element.setAttribute(attribute[i][0], "");
            }
        }
    }

    if (child instanceof Array) {
        for (const i = 0; i < child.length; i++) {
            if (child[i] instanceof Node) {
                element.appendChild(child[i]);
            } else {
                element.appendChild(document.createTextNode(child[i]));
            }
        }
    }

    return element;
}

const toaster = {
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

const keyboardHandlers = {
    storyListing: function() {
        const cards = Array.from(document.querySelectorAll("#container > main > article.card"));
        const cardCount = cards.length - 1;

        let selected = -1;

        function select(index) {
            const element = cards[index];

            element.classList.add("selected");

            const position = (element.getBoundingClientRect().top - document.body.getBoundingClientRect().top) - 60;

            window.scrollTo({
                top: position,
                behavior: "smooth"
            });
        }

        Mousetrap.bind("esc", function(event) {
            event.preventDefault();

            cards.forEach(card => {
                card.classList.remove("selected");
            });

            selected = -1;
        });
        Mousetrap.bind("enter", function(event) {
            event.preventDefault();

            const selected = Array.from(document.querySelectorAll("#container > main > article.card.selected > .card__title > .media-object > .media-object__title > .media-object__title--sup > a"));

            const element = selected[0];

            window.location.href = element.getAttribute("href");
        });

        Mousetrap.bind("ctrl+up", function(event) {
            event.preventDefault();

            if (selected != 0) {
                cards.forEach(card => {
                    card.classList.remove("selected");
                });

                selected -= 1;

                select(selected);
            }
        });
        Mousetrap.bind("ctrl+down", function(event) {
            event.preventDefault();

            if (selected != cardCount) {
                cards.forEach(card => {
                    card.classList.remove("selected");
                });

                selected += 1;

                select(selected);
            }
        });
    },
};
