import dateformat from "dateformat";
import m from "mithril";

import { humanize } from "../utils";
import { IStory } from "../models/story";

interface IComponentStory {
    story: IStory;
}

export class ComponentStory implements m.ClassComponent<IComponentStory> {
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
                        m("div", {
                            "part": "top left",
                            "bg": ((type) => {
                                switch (type) {
                                    case "explicit":
                                        return "black-500";
                                    case "mature":
                                        return "red-500";
                                    case "teen":
                                        return "green-600";
                                    default:
                                        return "gray-700";
                                }
                            })(vnode.attrs.story.square.rating),
                        }),
                        m("div", {
                            "part": "top right",
                            "bg": ((type) => {
                                switch (type) {
                                    case "using":
                                        return "orange-500";
                                    default:
                                        return "gray-700";
                                }
                            })(vnode.attrs.story.square.warnings),
                        }),
                        m("div", {
                            "part": "bottom center",
                            "bg": ((type) => {
                                switch (type) {
                                    case "completed":
                                        return "green-600";
                                    case "in-progress":
                                        return "blue-500";
                                    case "hiatus":
                                        return "purple-500";
                                    default:
                                        return "red-500";
                                }
                            })(vnode.attrs.story.square.state),
                        }),
                    ]),
                    m("div", [
                        m("h3", [
                            m(m.route.Link, { "href": `/story/${vnode.attrs.story.id}/1` }, vnode.attrs.story.name),
                            " by ",
                            authors,
                            // " ",
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
                    vnode.attrs.story.tags.map((t) => m(m.route.Link, {
                        "key": t.id,
                        "label": true,
                        "href": `/tag/${t.id}/1`,
                        "bg": ((type) => {
                            switch (type) {
                                case "warning":
                                    return "red-500";
                                case "pairing":
                                    return "orange-500";
                                case "character":
                                    return "purple-500";
                                default:
                                    return "gray-700";
                            }
                        })(t.type),
                    }, t.name)),
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
