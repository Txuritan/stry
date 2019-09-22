use {
    crate::{Message, Model},
    seed::{a, attrs, div, nav, prelude::*},
};

pub(crate) fn view(model: &Model) -> Vec<Node<Message>> {
    vec![nav![
        attrs! { "nav" => "primary", "flex" => "grow", "bg" => "black-800" },
        div![
            attrs! { "nav-section" => "grow" },
            div![attrs! { "nav-item" => true }, div!["mode:"]],
            div![
                attrs! { "nav-item" => if !model.data.dark { "brand" } else { "" } },
                a![simple_ev(Ev::Click, Message::ToggleTheme), "light",],
            ],
            div![
                attrs! { "nav-item" => if model.data.dark { "brand" } else { "" } },
                a![simple_ev(Ev::Click, Message::ToggleTheme), "dark",],
            ],
        ],
        div![
            attrs! { "nav-section" => true },
            div![
                attrs! { "nav-item" => "brand" },
                a![
                    attrs! { "href" => "#!/home/1" },
                    /*"v2.1.0-fab195f-modified"*/
                    concat!("v", env!("CARGO_PKG_VERSION"), "-", env!("GIT_VERSION"))
                ],
            ],
        ],
    ]]
}
