use {
    crate::{Message, Model},
    common::models::{TagType, Story, Rating, State, Warning},
    seed::{a, attrs, div, h3, p, prelude::*, span},
};

pub(crate) fn view(model: &Model, story: &Story) -> Node<Message> {
    div![
        attrs!{"panel" => true},
        div![
            attrs!{"head" => true, "flex" => true},
            div![
                attrs!{"flex" => true, "flex-item" => "grow"},
                div![
                    attrs!{"square" => true, "mar-right" => "normal"},
                    div![
                        attrs!{
                            "part" => "top left", 
                            "bg" => match story.square.rating {
                                Rating::Explicit => "black-500",
                                Rating::Mature => "red-500",
                                Rating::Teen => "green-600",
                                Rating::General => "gray-700",
                            }
                        }
                    ],
                    div![
                        attrs!{
                            "part" => "top right",
                            "bg" => match story.square.warnings {
                                Warning::Using => "orange-500",
                                Warning::None => "gray-700",
                            }
                        }
                    ],
                    div![
                        attrs!{
                            "part" => "bottom center",
                            "bg" => match story.square.state {
                                State::Completed => "green-600",
                                State::InProgress => "blue-500",
                                State::Hiatus => "purple-500",
                                State::Abandoned => "red-500",
                            }
                        }
                    ]
                ],
                div![
                    h3![
                        a![attrs!{"href" => format!("#!/story/{}/1", story.id)}, story.name],
                        "by",
                        story.authors.iter().enumerate().fold(vec![], |mut vec, (i, author)| {
                            vec.push(a![attrs!{"href" => format!("#!/author/{}/1", author.id)}, author.name]);

                            if i != story.authors.len() - 1 {
                                vec.push(span![","]);
                            }

                            vec
                        }),
                    ],
                    p![
                        story.origins.iter().enumerate().fold(vec![], |mut vec, (i, origin)| {
                            vec.push(a![attrs!{"href" => format!("#!/origin/{}/1", origin.id)}, origin.name]);

                            if i != story.origins.len() - 1 {
                                vec.push(span![","]);
                            }

                            vec
                        }),
                    ]
                ]
            ],
            p![attrs!{"flex-item" => "grow", "txt" => "right"}, story.updated.format("%b %e, %Y").to_string()]
        ],
        div![
            attrs!{"body" => true},
            p![story.summary],
            div![
                attrs!{"label-list" => true},
                story.tags.iter().map(|tag| {
                    a![
                        attrs!{
                            "href" => format!("#!/tag/{}/1", tag.id),
                            "label" => true,
                            "bg" => match tag.typ {
                                TagType::Warning => "red-500",
                                TagType::Pairing => "orange-500",
                                TagType::Character => "purple-500",
                                TagType::General => "gray-700",
                            }
                        },
                        tag.name
                    ]
                })
            ]
        ],
        div![
            attrs!{"foot" => true},
            div![
                attrs!{"flex" => true},
                p![attrs!{"flex-item" => "grow", "txt" => "small"}],
                p![
                    attrs!{"flex-item" => "grow", "txt" => "small right"},
                    format!("{} | {} words | {} chapters", story.language, story.words, story.chapters)
                ]
            ]
        ]
    ]
}
