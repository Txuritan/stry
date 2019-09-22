import m from "mithril";

import { humanize } from "../utils";
import { ComponentMain } from "../components/main";
import { IResponse, ITagResponse } from "../data";

export interface ITagList {
    type: string;
}

export class PageTagList implements m.ClassComponent<ITagList> {
    private data: IResponse<ITagResponse> | null = null;

    fetch(type: string, page: number) {
        if (page <= 0) {
            page = 1;
        }

        m.request<IResponse<ITagResponse>>({
            url: `/api/:type/:page`,
            params: {
                type: type,
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

    oninit(vnode: m.CVnode<ITagList>) {
        let page = Number(m.route.param("page"));

        this.fetch(m.route.param("type"), page);
    }

    view(vnode: m.CVnode<ITagList>) {
        let page = Number(m.route.param("page"));

        if (page <= 0) {
            page = 1;
        }

        let body: m.ChildArray = [];

        if (this.data != null) {
            let api = this.data;

            // let rows: m.ChildArray = [];

            body.push(m("div", { "key": "list", "label-list": true }, [
                api.data.tags.map((t) => m(m.route.Link, {
                    "key": t.id,
                    "label": true,
                    "href": `/tag/${t.id}/1`,
                    "bg": ((type) => {
                        switch (type) {
                            case "warning":
                                return "red-500";
                            case "paring":
                                return "orange-500";
                            case "character":
                                return "purple-500";
                            default:
                                return "gray-700";
                        }
                    })(t.type),
                }, t.name)),
            ]));

            // api.data.tags.forEach((t, i) => {
            //     rows.push(m("tr", { key: `${t.id}-tag` }, [
            //         m("td", m(m.route.Link, { href: `/tag/${t.id}/1` }, t.name)),
            //     ]));
            // });

            let prev = `/list/${m.route.param("type")}/${page == 1 ? 1 : page - 1}`;
            let next = `/list/${m.route.param("type")}/$${page == api.data.count ? page : page + 1}`;

            body.push(m("div", { "key": `pagination`, "pagination": true }, [
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

                        this.fetch(m.route.param("type"), page == 1 ? 1 : page - 1);
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

                        this.fetch(m.route.param("type"), page == api.data.count ? page : page + 1);
                    }
                }, "next"),
            ]));
        }

        return m(ComponentMain, body);
    }
}