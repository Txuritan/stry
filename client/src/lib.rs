mod commands;
mod components;
mod data;
mod pages;

use {
    crate::{
        components::{footer, navbar},
        data::SiteData,
        pages::{Page, PageData},
    },
    common::models::{Wrapper, StoryResponse},
    seed::{attrs, div, fetch, prelude::*},
};

#[derive(Clone, Debug)]
pub struct Model {
    data: SiteData,
    debug: bool,
    page: Page,
    page_data: PageData,
    page_number: u32,
    search_query: String,
}

impl Default for Model {
    fn default() -> Model {
        Model {
            data: SiteData::get(),
            debug: cfg!(debug_assertions),
            page: Page::Home,
            page_data: PageData::default(),
            page_number: 1,
            search_query: String::new(),
        }
    }
}

#[derive(Clone)]
enum Message {
    StoreData,

    ToggleTheme,

    ChangePage(Page),
    ChangePageNumber(u32),

    FetchStories,
    FetchedStories(fetch::ResponseDataResult<Wrapper<StoryResponse>>),
}

fn init(url: seed::Url, orders: &mut impl Orders<Message>) -> Model {
    Model::default()
}

fn update(msg: Message, model: &mut Model, orders: &mut impl Orders<Message>) {
    match msg {
        // Non-render Events
        Message::StoreData => match seed::storage::get_storage() {
            Some(storage) => {
                model.data.store(&storage);

                orders.skip();
            },
            None => {
                log::error!("Unable to get local storage, does this browser support it");
            }
        },

        // Re-render events
        Message::ToggleTheme => {
            model.data.dark = !model.data.dark;

            let dark = model.data.dark;
            web_sys::window().map(|window| {
                window.document().map(|document| {
                    document.body().map(|body| {
                        let _ = body.set_attribute("theme", if dark { "dark" } else { "light" });
                    });
                });
            });

            orders.send_msg(Message::StoreData);
        },
        Message::ChangePage(page) => model.page = page,
        Message::ChangePageNumber(number) => model.page_number = number,

        // Fetch events
        Message::FetchStories => {
            orders.skip().perform_cmd(commands::fetch_stories(model.page_number));
        },
        Message::FetchedStories(fetched) => match fetched {
            Ok(stories) => {
                orders.skip();
            },
            Err(err) => {
                log::error!("{}", format!("Fetch error - Fetching stories failed - {:#?}", err));

                orders.skip();
            },
        }
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
                match model.page {
                    Page::Home => pages::home::view(model),
                    _ => pages::not_found(model),
                }
            ],
            footer::view(model),
        ],
        div![attrs! { "grid-column" => "1" }],
    ]
}

fn routes(url: seed::Url) -> Message {
    if url.path.is_empty() {
        return Message::ChangePage(Page::Home)
    }

    Message::ChangePage(Page::Home)
}

#[wasm_bindgen(start)]
pub fn render() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug).message_on_new_line());

    seed::App::build(init, update, view)
        .mount(seed::body())
        .routes(routes)
        .finish()
        .run();
}
