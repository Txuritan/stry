import m from "mithril";

import { humanize } from "../utils";
import { ComponentMain } from "../components/main";
import { ComponentStory } from "../components/story";
import { IResponse, IStoryResponse } from "../data";

export interface IPageList {
    type: string;
}

export class PageStoryList implements m.ClassComponent<IPageList> {
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

            document.title = `${page} | ${m.route.param("type")} | stry`;

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