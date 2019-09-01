import m from "mithril";
import marked from "marked";

import { humanize } from "../utils";
import { ComponentMain } from "../components/main";
import { ComponentStory } from "../components/story";
import { IResponse, IChapterResponse } from "../data";

export interface IPageChapter {
    type: string;
}

export class PageChapter implements m.ClassComponent<IPageChapter> {
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
        if (this.api != null) {
            let data = this.api.data;

            let page = Number(m.route.param("page"));

            if (page <= 0) {
                page = 1;
            }

            let prev = `/story/${m.route.param("id")}/${page == 1 ? 1 : page - 1}`;
            let next = `/story/${m.route.param("id")}/${page == data.story.chapters ? page : page + 1}`;

            return m(ComponentMain, [
                    m(ComponentStory, { story: data.story }),

                    m("hr"),

                    m("p", `Chapter ${page}: ${data.chapter.name}`),

                    m("hr"),

                    m("div", { "pad": "p" }, [
                        m.trust(marked(data.chapter.raw)),
                    ]),

                    m("hr"),

                    m("div", { "pagination": true }, [
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

                                this.fetch(m.route.param("id"), page == 1 ? 1 : page - 1);
                            }
                        }, "prev"),

                        m("p", { "page": true }, `${page} / ${humanize(data.story.chapters)}`),

                        m(m.route.Link, {
                            "pager": `right ${page == data.story.chapters ? "disabled" : ""}`,
                            "href": next,
                            onclick: () => {
                                if (this.api != null) {
                                    window.scrollTo({ top: 0, behavior: 'smooth' });

                                    m.route.set(next, null, {
                                        state: {
                                            key: next,
                                        },
                                    });

                                    this.fetch(m.route.param("id"), page == data.story.chapters ? page : page + 1);
                                }
                            }
                        }, "next"),
                    ])
            ]);
        } else {
            return [];
        }
    }
}