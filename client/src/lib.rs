mod components;

use {
    crate::components::{footer, navbar},
    seed::{attrs, div, prelude::*},
};

#[derive(Clone, Debug)]
pub struct Model {
    dark: bool,
    debug: bool,
}

impl Default for Model {
    fn default() -> Model {
        Model {
            dark: false,
            debug: cfg!(debug_assertions),
        }
    }
}

#[derive(Clone)]
pub enum Message {
    ToggleTheme,
}

fn update(msg: Message, model: &mut Model, orders: &mut impl Orders<Message>) {
    match msg {
        Message::ToggleTheme => {
            model.dark = !model.dark;

            let dark = model.dark;
            web_sys::window().map(|window| {
                window.document().map(|document| {
                    document.body().map(|body| {
                        let _ = body.set_attribute("theme", if dark { "dark" } else { "light" });
                    });
                });
            });
        },
    }
}

fn view(model: &Model) -> impl View<Message> {
    div![
        attrs! { "grid" => "auto" },
        div![attrs! { "grid-column" => "1" }],
        div![
            attrs! { "grid-column" => "10 max", "shadow" => true },
            navbar::view(model),
            div![
                attrs! { "l-bg" => "gray-100", "d-bg" => "black-700", "pad" => "content" },
            ],
            footer::view(model),
        ],
        div![attrs! { "grid-column" => "1" }],
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug).message_on_new_line());

    let app = seed::App::build(|_, _| Model::default(), update, view)
        .mount(seed::body())
        .finish()
        .run();
}
