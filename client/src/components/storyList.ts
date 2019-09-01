import m from "mithril";

import { humanize } from "../utils";
import { ComponentMain } from "./main";
import { ComponentStory } from "./story";
import { IResponse, IStoryResponse } from "../data";

export interface IComponentStoryList {
    api: IResponse<IStoryResponse> | null;
    page: number;
    url: string;
}

export class ComponentStoryList implements m.ClassComponent<IComponentStoryList> {
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