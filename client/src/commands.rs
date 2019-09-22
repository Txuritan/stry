use {
    crate::Message,
    futures::Future,
    seed::{Method, Request},
};

pub(crate) fn fetch_stories(page: u32) -> impl Future<Item = Message, Error = Message> {
    Request::new(format!("/api/stories/{}", page)).fetch_json_data(Message::FetchedStories)
}