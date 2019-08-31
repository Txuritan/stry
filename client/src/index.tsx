declare let GIT_VERSION: string;

const style = require("./index.scss");

import dateformat from "dateformat";
// import dexie from "dexie";
// import dompurify from "dompurify";
import marked from "marked";
import m from "mithril";

function humanize(n: number) {
    let str = n.toString().split('.');

    str[0] = str[0].replace(/(\d)(?=(\d\d\d)+(?!\d))/g, '$1' + ',');

    return str.join('.');
}

enum Mode {
    Light,
    Dark,
}

let state = {
    mode: Mode.Light,
};

class Color {
    light: string;
    dark: string;

    constructor({ light, dark }: { light: string, dark: string }) {
        this.light = light;
        this.dark = dark;
    }

    toString() {
        return state.mode === Mode.Light ? this.light : this.dark;
    }
}

interface IAuthor {
    id: string;
    name: string;
    created: string;
    updated: string;
}

interface IOrigin {
    id: string;
    name: string;
    created: string;
    updated: string;
}

interface ITag {
    id: string;
    name: string;
    type: string;
    created: string;
    updated: string;
}

interface IStory {
    id: string;
    name: string;
    summary: string;
    language: string;
    square: IStorySquare;
    chapters: number;
    words: number;
    authors: IAuthor[];
    origins: IOrigin[];
    tags: ITag[];
    series: null;
    created: string;
    updated: string;
}

interface IStorySquare {
    rating: string;
    warnings: string;
    state: string;
}

interface IChapter {
    id: string;
    name: string;
    raw: string;
    words: number;
    created: string;
    updated: string;
}

interface IResponse<T> {
    data: T;
}

interface IStoryResponse {
    count: number;
    pages: number;
    stories: IStory[];
}

interface IChapterResponse {
    chapter: IChapter;
    story: IStory;
}

class ComponentNavbar implements m.ClassComponent {
    view(vnode: m.CVnode) {
        return [
            m("div", { "nav": "primary", "flex": "grow" }, [
                m("div", { "nav-section": "grow" }, [
                    m("div", { "nav-item": "brand" }, [
                        m(m.route.Link, { "href": "/home/1" }, "stry"),
                    ]),
                ]),
                m("div", { "nav-section": true }, [
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

class ComponentFooter implements m.ClassComponent {
    view(vnode: m.CVnode) {
        return [
            m("div", { "nav": "primary", "flex": "grow" }, [
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

class ComponentMain implements m.ClassComponent {
    view(vnode: m.CVnode) {
        return [
            m("div", { "grid": "auto" }, [
                m("div", { "grid-column": "1" }),
                m("div", { "grid-column": "10 max" }, [
                    m(ComponentNavbar),
                ]),
                m("div", { "grid-column": "1" }),
            ]),
            m("div", { "grid": "auto" }, [
                m("div", { "grid-column": "1" }),
                m("div", { "grid-column": "10 max", "bg": "white", "pad": "content" }, vnode.children),
                m("div", { "grid-column": "1" }),
            ]),
            m("div", { "grid": "auto" }, [
                m("div", { "grid-column": "1" }),
                m("div", { "grid-column": "10 max" }, [
                    m(ComponentFooter),
                ]),
                m("div", { "grid-column": "1" }),
            ]),
        ];
    }
}

interface IComponentStory {
    story: IStory;
}

class ComponentStory implements m.ClassComponent<IComponentStory> {
    view(vnode: m.CVnode<IComponentStory>) {
        let authors: m.Children[] = [];

        vnode.attrs.story.authors.forEach((a, i) => {
            authors.push(m(m.route.Link, { "key": `${a.id}-author`, "href": `/author/${a.id}/1` }, a.name));

            if (i != vnode.attrs.story.authors.length - 1) {
                authors.push(", ");
            }
        });

        let origins: m.Children[] = [];

        vnode.attrs.story.origins.forEach((o, i) => {
            origins.push(m(m.route.Link, { "key": `${o.id}-author`, "href": `/origin/${o.id}/1` }, o.name));

            if (i != vnode.attrs.story.origins.length - 1) {
                origins.push(m("span", { "key": `${o.id}-span` }, ", "));
            }
        });

        return m("div", { "panel": true }, [
            m("div", { "head": true, "flex": true }, [
                m("div", { "flex": true, "flex-item": "grow" }, [
                    m("div", { "square": true, "mar-right": "normal" }, [
                        m("div", { "part": "top left", "bg": vnode.attrs.story.square.rating }),
                        m("div", { "part": "top right", "bg": vnode.attrs.story.square.warnings }),
                        m("div", { "part": "bottom center", "bg": vnode.attrs.story.square.state }),
                    ]),
                    m("div", [
                        m("h3", [
                            m(m.route.Link, { "href": `/story/${vnode.attrs.story.id}/1` }, vnode.attrs.story.name),
                            " by ",
                            authors,
                            " ",
                            // m(m.route.Link, { "txt": "small", "href": `/story/${vnode.attrs.story.id}/reviews/1` }, "[reviews]"),
                        ]),
                        m("p", origins),
                    ]),
                ]),
                m("p", { "flex-item": "grow", "txt": "right" }, dateformat(vnode.attrs.story.updated, "mmm d, yyyy")),
            ]),
            m("div", { "body": true }, [
                m("p", vnode.attrs.story.summary),
                m("div", { "label-list": true }, [
                    vnode.attrs.story.tags.map((t) => m(m.route.Link, { "key": t.id, "label": t.type, "href": `/tag/${t.id}/1` }, t.name)),
                ]),
            ]),
            m("div", { "foot": true }, [
                m("div", { "flex": true }, [
                    m("p", { "flex-item": "grow", "txt": "small" }, ``),
                    m("p", { "flex-item": "grow", "txt": "small right" }, `${vnode.attrs.story.language} | ${humanize(vnode.attrs.story.words)} words | ${humanize(vnode.attrs.story.chapters)} chapters`),
                ]),
            ]),
        ]);
    }
}

interface IComponentStoryList {
    api: IResponse<IStoryResponse> | null;
    page: number;
    url: string;
}

class ComponentStoryList implements m.ClassComponent<IComponentStoryList> {
    view(vnode: m.CVnode<IComponentStoryList>) {
        let stories: m.ChildArray = [];

        if (vnode.attrs.api != null) {
            let api = vnode.attrs.api;

            api.data.stories.forEach((s, i) => {
                stories.push(m(ComponentStory, { key: `${s.id}-story`, story: s }));

                stories.push(m("hr", { "key": `${s.id}-hr` }));
            });

            let prev = `/${vnode.attrs.url}/${vnode.attrs.page == 1 ? 1 : vnode.attrs.page - 1}`;
            let next = `/${vnode.attrs.url}/${vnode.attrs.page + 1}`;

            stories.push(m("div", { "key": `pagination`, "pagination": true }, [
                m(m.route.Link, {
                    "pager": `left ${vnode.attrs.page == 1 ? "disabled" : ""}`,
                    "href": prev,
                    onclick: () => m.route.set(prev, null, {
                        state: {
                            key: prev,
                        },
                    })
                }, "prev"),
                m("p", { "page": true }, `${vnode.attrs.page} / ${humanize(api.data.pages)}`),
                m(m.route.Link, {
                    "pager": `right ${vnode.attrs.page == api.data.pages ? "disabled" : ""}`,
                    "href": next,
                    onclick: () => m.route.set(next, null, {
                        state: {
                            key: next,
                        },
                    })
                }, "next"),
            ]));
        }

        return m(ComponentMain, stories);
    }
}

class Page404 implements m.ClassComponent {
    view(vnode: m.CVnode) {
        return m("h3", "404: Page Not Found");
    }
}

class PageHome implements m.ClassComponent {
    private api: IResponse<IStoryResponse> | null = null;

    oninit(vnode: m.CVnode) {
        let page = Number(m.route.param("key"));

        if (page <= 0) {
            page = 1;
        }

        m.request<IResponse<IStoryResponse>>({
            url: `/api/stories/:page`,
            params: {
                page: page,
            },
        })
            .then((api) => {
                this.api = api;
            })
            .catch((err) => {
                console.error(err);
            });
    }

    view(vnode: m.CVnode) {
        let page = Number(m.route.param("key"));

        if (page <= 0) {
            page = 1;
        }

        return m(ComponentStoryList, { api: this.api, page: page, url: "home" });
    }
}

interface IPageChapter {
    type: string;
}

class PageChapter implements m.ClassComponent<IPageChapter> {
    private api: IResponse<IChapterResponse> | null = null;

    fetch(id: string, page: number) {
        if (page <= 0) {
            page = 1;
        }

        m.request<IResponse<IChapterResponse>>({
            url: `/api/story/:id/chapter/:page`,
            params: {
                id: id,
                page: page,
            },
        })
            .then((api) => {
                this.api = api;
            })
            .catch((err) => {
                console.error(err);
            });
    }

    oninit(vnode: m.CVnode<IPageChapter>) {
        let page = Number(m.route.param("page"));

        if (page <= 0) {
            page = 1;
        }

        this.fetch(m.route.param("id"), page);
    }

    view(vnode: m.CVnode<IPageChapter>) {
        let page = Number(m.route.param("page"));

        if (page <= 0) {
            page = 1;
        }

        return m(ComponentMain, [
            this.api != null ? [
                m(ComponentStory, { story: this.api.data.story }),

                m("hr"),

                m("p", `Chapter ${page}: ${this.api.data.chapter.name}`),

                m("hr"),

                m("div", { "pad": "p" }, [
                    m.trust(marked(this.api.data.chapter.raw)),
                ]),

                m("hr"),

                m("div", { "pagination": true }, [
                    m(m.route.Link, {
                        "pager": `left ${page == 1 ? "disabled" : ""}`,
                        "href": `/story/${m.route.param("id")}/${page == 1 ? 1 : page - 1}`,
                        onclick: () => {
                            window.scrollTo({ top: 0, behavior: 'smooth' });

                            m.route.set(`/story/${m.route.param("id")}/${page == 1 ? 1 : page - 1}`, null, {
                                state: {
                                    key: `/story/${m.route.param("id")}/${page == 1 ? 1 : page - 1}`,
                                },
                            });

                            this.fetch(m.route.param("id"), page == 1 ? 1 : page - 1);
                        }
                    }, "prev"),

                    m("p", { "page": true }, `${page} / ${humanize(this.api.data.story.chapters)}`),

                    m(m.route.Link, {
                        "pager": `right ${page == this.api.data.story.chapters ? "disabled" : ""}`,
                        "href": `/story/${m.route.param("id")}/${page == this.api.data.story.chapters ? page : page + 1}`,
                        onclick: () => {
                            if (this.api != null) {
                                window.scrollTo({ top: 0, behavior: 'smooth' });

                                m.route.set(`/story/${m.route.param("id")}/${page == this.api.data.story.chapters ? page : page + 1}`, null, {
                                    state: {
                                        key: `/story/${m.route.param("id")}/${page == this.api.data.story.chapters ? page : page + 1}`,
                                    },
                                });

                                this.fetch(m.route.param("id"), page == this.api.data.story.chapters ? page : page + 1);
                            }
                        }
                    }, "next"),
                ])
            ] : [],
        ]);
    }
}

interface IPageList {
    type: string;
}

class PageStoryList implements m.ClassComponent<IPageList> {
    private data: IResponse<IStoryResponse> | null = null;

    fetch(type: string, id: string, page: number) {
        if (page <= 0) {
            page = 1;
        }

        m.request<IResponse<IStoryResponse>>({
            url: `/api/:type/:id/:page`,
            params: {
                type: type,
                id: id,
                page: page,
            },
        })
            .then((api) => {
                this.data = api;
            })
            .catch((err) => {
                console.error(err);
            });
    }

    oninit(vnode: m.CVnode<IPageList>) {
        let page = Number(m.route.param("page"));

        this.fetch(m.route.param("type"), m.route.param("id"), page);
    }

    view(vnode: m.CVnode<IPageList>) {
        let page = Number(m.route.param("page"));

        if (page <= 0) {
            page = 1;
        }

        let stories: m.ChildArray = [];

        if (this.data != null) {
            let api = this.data;

            api.data.stories.forEach((s, i) => {
                stories.push(m(ComponentStory, { key: `${s.id}-story`, story: s }));

                stories.push(m("hr", { "key": `${s.id}-hr` }));
            });

            let prev = `/${m.route.param("type")}/${m.route.param("id")}/${page == 1 ? 1 : page - 1}`;
            let next = `/${m.route.param("type")}/${m.route.param("id")}/${page == api.data.count ? page : page + 1}`;

            stories.push(m("div", { "key": `pagination`, "pagination": true }, [
                m(m.route.Link, {
                    "pager": `left ${page == 1 ? "disabled" : ""}`,
                    "href": prev,
                    onclick: () => {
                        window.scrollTo({ top: 0, behavior: 'smooth' });

                        m.route.set(prev, null, {
                            state: {
                                key: prev,
                            },
                        });

                        this.fetch(m.route.param("type"), m.route.param("id"), page == 1 ? 1 : page - 1);
                    }
                }, "prev"),

                m("p", { "page": true }, `${page} / ${humanize(api.data.pages)}`),

                m(m.route.Link, {
                    "pager": `right ${page == api.data.pages ? "disabled" : ""}`,
                    "href": next,
                    onclick: () => {
                        window.scrollTo({ top: 0, behavior: 'smooth' });

                        m.route.set(next, null, {
                            state: {
                                key: next,
                            },
                        });

                        this.fetch(m.route.param("type"), m.route.param("id"), page == api.data.count ? page : page + 1);
                    }
                }, "next"),
            ]));
        }

        return m(ComponentMain, stories);
    }
}

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
