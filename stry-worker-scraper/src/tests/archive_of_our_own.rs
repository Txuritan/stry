use {
    crate::{
        archive_of_our_own,
        models::{Chapter, Details, Language, Rating, State},
    },
    chrono::prelude::*,
};

const MULTIPLE_CHAPTERS_HTML: &str =
    include_str!("./data/archive-of-our-own/multiple-chapters.html");
const MULTIPLE_CHAPTERS_MARKDOWN: &str =
    include_str!("./data/archive-of-our-own/multiple-chapters.md");

const SINGLE_CHAPTER_HTML: &str = include_str!("./data/archive-of-our-own/single-chapter.html");
const SINGLE_CHAPTER_MARKDOWN: &str = include_str!("./data/archive-of-our-own/single-chapter.md");

#[test]
#[allow(non_snake_case)]
fn multiple_chapters__chapter() {
    assert_eq!(
        Chapter {
            name: String::from("Before"),
            main: String::from(MULTIPLE_CHAPTERS_MARKDOWN),
            words: 6504,
            pre: String::new(),
            post: String::new(),
        },
        archive_of_our_own::get_chapter(MULTIPLE_CHAPTERS_HTML, 1).unwrap()
    );
}

#[test]
#[allow(non_snake_case)]
fn multiple_chapters__details() {
    assert_eq!(Details {
        name: String::from("Fellow Traveler"),
        summary: String::from("No regime, no matter how brutal, can last without the tacit acceptance of the majority. Of people who close their eyes to terrible things as long as they are not affected by them. Of people who could do something about things they disapprove of, but choose not to. Of people like Rhea Jag, who just want to do well on their exams and maintain their conditional acceptance into university. The Seventy-Fourth Hunger Games are approaching, however, and soon, it will be impossible to remain apolitical. Twoshot. "),
        chapters: 2,
        language: Language::English,
        rating: Rating::Mature,
        state: State::Completed,
        authors: vec![String::from("quiet_wraith")],
        origins: vec![String::from("Hunger Games Series - All Media Types")],
        tags: vec![],
        created: Utc.ymd(2019, 9, 27).and_hms(0, 0, 0),
        updated: Utc.ymd(2019, 10, 4).and_hms(0, 0, 0),
    }, archive_of_our_own::get_details(MULTIPLE_CHAPTERS_HTML).unwrap());
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
        archive_of_our_own::get_chapter(SINGLE_CHAPTER_HTML, 1).unwrap()
    );
}

#[test]
#[allow(non_snake_case)]
fn single_chapter__details() {
    assert_eq!(Details {
        name: String::from("Little Cog"),
        summary: String::from("When he was twelve years old, Dey Brown joined the Peacekeeper Academy.\n\nWhen he was seventeen years old, the Rebellion broke out.\n\nWhen he was ninety-three years old, his past was unearthed.\n\nIt is doubtful that even a tenth of a percent of the Peacekeepers faced any sort of justice, but perhaps, just perhaps, there was a steady trickle of cases to remind them that their crimes were not forgotten. "),
        chapters: 1,
        language: Language::English,
        rating: Rating::Teen,
        state: State::Completed,
        authors: vec![String::from("quiet_wraith")],
        origins: vec![String::from("Hunger Games Series - All Media Types")],
        tags: vec![],
        created: Utc.ymd(2020, 1, 5).and_hms(0, 0, 0),
        updated: Utc.ymd(2020, 1, 5).and_hms(0, 0, 0),
    }, archive_of_our_own::get_details(SINGLE_CHAPTER_HTML).unwrap());
}
