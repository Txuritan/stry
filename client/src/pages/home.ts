import m from "mithril";

import { ComponentStoryList } from "../components/storyList";
import { IResponse, IStoryResponse } from "../data";

export class PageHome implements m.ClassComponent {
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