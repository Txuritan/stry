declare let GIT_VERSION: string;

import m from "mithril";

import { state } from "../state";

export class ComponentFooter implements m.ClassComponent {
    view(vnode: m.CVnode) {
        return [
            m("div", { "nav": "primary", "flex": "grow", "bg": "black-800" }, [
                m("div", { "nav-section": "grow" }, [
                    m("div", { "nav-item": true }, [
                        m("div", "mode: "),
                    ]),
                    m("div", { "nav-item": state.darkMode ? true : "brand" }, [
                        m("a", {
                            "onclick": () => {
                                state.inverse();
                            }
                        }, "light"),
                    ]),
                    m("div", { "nav-item": state.darkMode ? "brand" : true }, [
                        m("a", {
                            "onclick": () => {
                                state.inverse();
                            }
                        }, "dark"),
                    ]),
                ]),
                m("div", { "nav-section": true }, [
                    m("div", { "nav-item": "brand" }, [
                        m(m.route.Link, { "href": "/home/1" }, GIT_VERSION),
                    ]),
                ]),
            ]),
        ];
    }
}