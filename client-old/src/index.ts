const style = require("./index.scss");

import m from "mithril";

import { PageChapter, PageHome, PageSearch, PageStoryList, PageTagList } from "./pages";

m.route(document.body, "/home/1", {
    "/home/:key": PageHome,
    "/search/:page": {
        onmatch: () => { return PageSearch; },
        render: (vnode) => {
            vnode.key = window.location.hash;

            return vnode;
        }
    },
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
    
    "/list/authors/:page": {
        onmatch: () => { return PageTagList; },
        render: (vnode) => {
            vnode.key = window.location.hash;

            vnode.attrs.type = "authors";

            return vnode;
        }
    },
    "/list/characters/:page": {
        onmatch: () => { return PageTagList; },
        render: (vnode) => {
            vnode.key = window.location.hash;

            vnode.attrs.type = "characters";

            return vnode;
        }
    },
    "/list/origins/:page": {
        onmatch: () => { return PageTagList; },
        render: (vnode) => {
            vnode.key = window.location.hash;

            vnode.attrs.type = "origins";

            return vnode;
        }
    },
    "/list/parings/:page": {
        onmatch: () => { return PageTagList; },
        render: (vnode) => {
            vnode.key = window.location.hash;

            vnode.attrs.type = "parings";

            return vnode;
        }
    },
    "/list/tags/:page": {
        onmatch: () => { return PageTagList; },
        render: (vnode) => {
            vnode.key = window.location.hash;

            vnode.attrs.type = "tags";

            return vnode;
        }
    },
    "/list/warnings/:page": {
        onmatch: () => { return PageTagList; },
        render: (vnode) => {
            vnode.key = window.location.hash;

            vnode.attrs.type = "warnings";

            return vnode;
        }
    },
});