use {
    crate::workers::scraper::{
        fanfiction,
        models::{Chapter, Details, Language, Rating, State},
    },
    chrono::prelude::*,
};

const MULTIPLE_CHAPTERS_HTML: &str = include_str!("./data/fanfiction/multiple-chapters.html");
const MULTIPLE_CHAPTERS_MARKDOWN: &str = include_str!("./data/fanfiction/multiple-chapters.md");

const SINGLE_CHAPTER_HTML: &str = include_str!("./data/fanfiction/single-chapter.html");
const SINGLE_CHAPTER_MARKDOWN: &str = include_str!("./data/fanfiction/single-chapter.md");

#[test]
#[allow(non_snake_case)]
fn multiple_chapters__chapter() {
    assert_eq!(
        Chapter {
            name: String::from("Before"),
            main: String::from(MULTIPLE_CHAPTERS_MARKDOWN),
            words: 6521,
            pre: String::new(),
            post: String::new(),
        },
        fanfiction::get_chapter(MULTIPLE_CHAPTERS_HTML).unwrap(),
    );
}

#[test]
#[allow(non_snake_case)]
fn multiple_chapters__details() {
    pretty_assertions::assert_eq!(Details {
        name: String::from("Fellow Traveler"),
        summary: String::from("It is not the fanatic who keeps a regime going, but the fellow traveler. A person willing to overlook terrible things. A person willing to condone terrible things. A person like Rhea Jag, who just wants to do well on her exams and doesn't care for politics. But the Seventy-Fourth Hunger Games draw near, and soon, it will be impossible to remain apolitical, if it ever was. Twoshot."),
        chapters: 2,
        language: Language::English,
        rating: Rating::Teen,
        state: State::Completed,
        authors: vec![String::from("quietwraith")],
        origins: vec![String::from("Hunger Games")],
        tags: vec![],
        created: Utc.ymd(2019, 9, 27).and_hms(22, 30, 35),
        updated: Utc.ymd(2019, 10, 4).and_hms(11, 44, 18),
    }, fanfiction::get_details(MULTIPLE_CHAPTERS_HTML).unwrap());
}

#[test]
#[allow(non_snake_case)]
fn single_chapter__chapter() {
    assert_eq!(
        Chapter {
            name: String::from("Little Cog"),
            main: String::from(SINGLE_CHAPTER_MARKDOWN),
            words: 1075,
            pre: String::new(),
            post: String::new(),
        },
        fanfiction::get_chapter(SINGLE_CHAPTER_HTML).unwrap(),
    );
}

#[test]
#[allow(non_snake_case)]
fn single_chapter__details() {
    pretty_assertions::assert_eq!(Details {
        name: String::from("Little Cog"),
        summary: String::from("When he was twelve years old, Dey Brown joined the Peacekeeper Academy. When he was seventeen years old, the Rebellion broke out. When he was ninety-three years old, his past was unearthed. It is doubtful that even a small percentage of the Peacekeepers faced any sort of justice, but perhaps there was a steady trickle of cases to remind them that their crimes were not forgotten."),
        chapters: 1,
        language: Language::English,
        rating: Rating::Teen,
        state: State::Completed,
        authors: vec![String::from("quietwraith")],
        origins: vec![String::from("Hunger Games")],
        tags: vec![],
        created: Utc.ymd(2020, 1, 5).and_hms(21, 0, 55),
        updated: Utc.ymd(2020, 1, 5).and_hms(21, 0, 55),
    }, fanfiction::get_details(SINGLE_CHAPTER_HTML).unwrap());
}
