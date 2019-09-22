pub(crate) mod home;

use {
    crate::{Message, Model},
    common::models::{Author, Origin, Tag, Story, Chapter},
    seed::{a, attrs, div, nav, prelude::*},
};

pub(crate) fn not_found(model: &Model) -> Vec<Node<Message>> {
    vec![div![]]
}

#[derive(Clone, Debug)]
pub(crate) enum Page {
    Home,
    Chapter,
    Search,

    AuthorList,
    OriginList,
    WarningList,
    PairingList,
    CharacterList,
    TagList,

    AuthorStories,
    OriginStories,
    WarningStories,
    PairingStories,
    CharacterStories,
    TagStories,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct PageData {
    authors: Option<Vec<Author>>,
    chapter: Option<Chapter>,
    origins: Option<Vec<Origin>>,
    stories: Option<Vec<Story>>,
    tags: Option<Vec<Tag>>,
}
