import m from "mithril";

import { ComponentFooter } from "./footer";
import { ComponentNavbar } from "./navbar";

export class ComponentMain implements m.ClassComponent {
    view(vnode: m.CVnode) {
        return [
            m("div", { "grid": "auto" }, [
                m("div", { "grid-column": "1" }),
                m("div", { "grid-column": "10 max", "shadow": true }, [
                    m(ComponentNavbar),
                    m("div", { "l-bg": "white", "d-bg": "black", "pad": "content" }, vnode.children),
                    m(ComponentFooter),
                ]),
                m("div", { "grid-column": "1" }),
            ]),
        ];
    }
}