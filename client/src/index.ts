const style = require("./index.scss");

import m from "mithril";

import { PageChapter, PageHome, PageStoryList } from "./pages";

document.addEventListener("stryStateDarkMode", ((e: CustomEvent<boolean>) => {
    document.body.setAttribute("theme", e.detail ? "dark" : "light")
}) as EventListener);

m.route(document.body, "/home/1", {
    "/home/:key": PageHome,
    "/story/:id/:page": {
        onmatch: () => { return PageChapter; },
        render: (vnode) => {
            vnode.key = window.location.hash;

            return vnode;
        }
    },
    "/author/:id/:page": {
        onmatch: () => { return PageStoryList; },
        render: (vnode) => {
            vnode.key = window.location.hash;

            vnode.attrs.type = "author";

            return vnode;
        }
    },
    "/origin/:id/:page": {
        onmatch: () => { return PageStoryList; },
        render: (vnode) => {
            vnode.key = window.location.hash;

            vnode.attrs.type = "origin";

            return vnode;
        }
    },
    "/tag/:id/:page": {
        onmatch: () => { return PageStoryList; },
        render: (vnode) => {
            vnode.key = window.location.hash;

            vnode.attrs.type = "tag";

            return vnode;
        }
    },
});