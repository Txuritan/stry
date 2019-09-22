use {
    crate::{Message, Model},
    seed::{a, attrs, div, input, nav, prelude::*},
};

pub fn view(model: &Model) -> Vec<Node<Message>> {
    vec![
        nav![
            attrs! { "nav" => "primary", "flex" => "grow", "bg" => if model.debug { "red-500" } else { "black-800" } },
            div![
                attrs! { "nav-section" => "grow" },
                div![
                    attrs! { "nav-item" => "brand" },
                    a![attrs! { "href" => "#!/home/1" }, "stry"],
                ],
            ],
            div![
                attrs! { "nav-section" => true },
                input![
                    attrs! { "nav-item" => true, "name" => "search", "type" => "search", "placeholder"=> "search", "l-bg" => "white-200", "d-bg" => "black-700" },
                ],
            ],
        ],
        nav![
            attrs! { "nav" => "secondary", "flex" => "grow", "bg" => "blue-500" },
            div![
                attrs! { "nav-section" => "grow" },
                div![
                    attrs! { "nav-item" => true },
                    a![attrs! { "href" => "#!/list/authors/1" }, "authors"]
                ],
                div![
                    attrs! { "nav-item" => true },
                    a![attrs! { "href" => "#!/list/origins/1" }, "origins"]
                ],
                div![
                    attrs! { "nav-item" => true },
                    a![attrs! { "href" => "#!/list/warnings/1" }, "warnings"]
                ],
                div![
                    attrs! { "nav-item" => true },
                    a![attrs! { "href" => "#!/list/pairings/1" }, "pairings"]
                ],
                div![
                    attrs! { "nav-item" => true },
                    a![attrs! { "href" => "#!/list/characters/1" }, "characters"]
                ],
                div![
                    attrs! { "nav-item" => true },
                    a![attrs! { "href" => "#!/list/tags/1" }, "tags"]
                ]
            ]
        ],
    ]
}
