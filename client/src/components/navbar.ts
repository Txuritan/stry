import m from "mithril";

export class ComponentNavbar implements m.ClassComponent {
    view(vnode: m.CVnode) {
        return [
            m("div", { "nav": "primary", "flex": "grow", "bg": "black" }, [
                m("div", { "nav-section": "grow" }, [
                    m("div", { "nav-item": "brand" }, [
                        m(m.route.Link, { "href": "/home/1" }, "stry"),
                    ]),
                ]),
                m("div", { "nav-section": true }, [
                    m("div", { "nav-item": true }, [
                        m(m.route.Link, { "href": "/list/author" }, "register"),
                    ]),
                    m("div", { "nav-item": true }, [
                        m(m.route.Link, { "href": "/list/author" }, "login"),
                    ]),
                    m("input", { "nav-item": true, "type": "search", "placeholder": "search" }),
                ]),
            ]),
            m("div", { "nav": "secondary", "flex": "grow", "bg": "blue" }, [
                m("div", { "nav-section": "grow" }, [
                    m("div", { "nav-item": true }, [
                        m(m.route.Link, { "href": "/list/author" }, "authors"),
                    ]),
                    m("div", { "nav-item": true }, [
                        m(m.route.Link, { "href": "/list/origin" }, "origins"),
                    ]),
                    m("div", { "nav-item": true }, [
                        m(m.route.Link, { "href": "/list/warning" }, "warnings"),
                    ]),
                    m("div", { "nav-item": true }, [
                        m(m.route.Link, { "href": "/list/pairing" }, "pairings"),
                    ]),
                    m("div", { "nav-item": true }, [
                        m(m.route.Link, { "href": "/list/character" }, "characters"),
                    ]),
                    m("div", { "nav-item": true }, [
                        m(m.route.Link, { "href": "/list/tag" }, "tags"),
                    ]),
                ]),
            ]),
        ];
    }
}