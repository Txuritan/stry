declare let DEBUG_MODE: Boolean;

import m from "mithril";

import { state } from "../state";

export class ComponentNavbar implements m.ClassComponent {
    oninit(vnode: m.CVnode) {
        state.checkQuery();
    }

    view(vnode: m.CVnode) {
        return [
            m("nav", { "nav": "primary", "flex": "grow", "bg": DEBUG_MODE ? "red-500" : "black-800" }, [
                m("div", { "nav-section": "grow" }, [
                    m("div", { "nav-item": "brand" }, [
                        m(m.route.Link, { "href": "/home/1" }, "stry"),
                    ]),
                ]),
                m("div", { "nav-section": true }, [
                    // m("div", { "nav-item": true }, [
                    //     m(m.route.Link, { "href": "/list/author" }, "register"),
                    // ]),
                    // m("div", { "nav-item": true }, [
                    //     m(m.route.Link, { "href": "/list/author" }, "login"),
                    // ]),
                    m("input", {
                        "nav-item": true,
                        "name": "search",
                        "type": "search",
                        "placeholder": "search",
                        "bg": state.darkMode ? "black-700" : "white-200",
                        oninput: ({ target }: { target: HTMLInputElement }) => { state.query = target.value; },
                        onkeyup: (e: KeyboardEvent) => {
                            if (e.keyCode === 13) {
                                m.route.set(m.buildPathname("/search/1", { search: state.query }), null, {
                                    state: {
                                        key: `/search/1-${state.query}`,
                                    },
                                });
                            } 
                        },
                        "value": state.query
                     }),
                ]),
            ]),
            m("nav", { "nav": "secondary", "flex": "grow", "bg": "blue-500" }, [
                m("div", { "nav-section": "grow" }, [
                    m("div", { "nav-item": true }, [
                        m(m.route.Link, { "href": "/list/authors/1" }, "authors"),
                    ]),
                    m("div", { "nav-item": true }, [
                        m(m.route.Link, { "href": "/list/origins/1" }, "origins"),
                    ]),
                    m("div", { "nav-item": true }, [
                        m(m.route.Link, { "href": "/list/warnings/1" }, "warnings"),
                    ]),
                    m("div", { "nav-item": true }, [
                        m(m.route.Link, { "href": "/list/pairings/1" }, "pairings"),
                    ]),
                    m("div", { "nav-item": true }, [
                        m(m.route.Link, { "href": "/list/characters/1" }, "characters"),
                    ]),
                    m("div", { "nav-item": true }, [
                        m(m.route.Link, { "href": "/list/tags/1" }, "tags"),
                    ]),
                ]),
            ]),
        ];
    }
}