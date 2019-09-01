declare let GIT_VERSION: string;

import m from "mithril";

export class ComponentFooter implements m.ClassComponent {
    view(vnode: m.CVnode) {
        return [
            m("div", { "nav": "primary", "flex": "grow", "bg": "black" }, [
                m("div", { "nav-section": "grow" }, [
                    m("div", { "nav-item": true }, [
                        m(m.route.Link, { "href": "/home/1" }, GIT_VERSION),
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