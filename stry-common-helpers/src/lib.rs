use {
    chrono::{TimeZone, Utc},
    stry_common::models::{
        Author, Character, Origin, Pairing, Rating, Square, State, Story, Tag, Warning,
    },
};

pub fn author(id: impl Into<String>, name: impl Into<String>) -> Author {
    Author {
        id: id.into(),

        name: name.into(),

        created: Utc.ymd(2020, 6, 8).and_hms(7, 22, 3),
        updated: Utc.ymd(2020, 6, 8).and_hms(7, 22, 3),
    }
}

pub fn character(id: impl Into<String>, name: impl Into<String>) -> Character {
    Character {
        id: id.into(),

        name: name.into(),

        created: Utc.ymd(2020, 6, 8).and_hms(7, 22, 3),
        updated: Utc.ymd(2020, 6, 8).and_hms(7, 22, 3),
    }
}

pub fn origin(id: impl Into<String>, name: impl Into<String>) -> Origin {
    Origin {
        id: id.into(),

        name: name.into(),

        created: Utc.ymd(2020, 6, 8).and_hms(7, 22, 3),
        updated: Utc.ymd(2020, 6, 8).and_hms(7, 22, 3),
    }
}

pub fn tag(id: impl Into<String>, name: impl Into<String>) -> Tag {
    Tag {
        id: id.into(),

        name: name.into(),

        created: Utc.ymd(2020, 6, 8).and_hms(7, 22, 3),
        updated: Utc.ymd(2020, 6, 8).and_hms(7, 22, 3),
    }
}

pub fn warning(id: impl Into<String>, name: impl Into<String>) -> Warning {
    Warning {
        id: id.into(),

        name: name.into(),

        created: Utc.ymd(2020, 6, 8).and_hms(7, 22, 3),
        updated: Utc.ymd(2020, 6, 8).and_hms(7, 22, 3),
    }
}

pub struct PairingBuilder {
    pub id: String,

    pub characters: Vec<Character>,

    pub platonic: bool,
}

impl PairingBuilder {
    pub fn with_character(mut self, id: impl Into<String>, name: impl Into<String>) -> Self {
        self.characters.push(Character {
            id: id.into(),

            name: name.into(),

            created: Utc.ymd(2020, 6, 8).and_hms(7, 22, 3),
            updated: Utc.ymd(2020, 6, 8).and_hms(7, 22, 3),
        });

        self
    }
}

pub struct StoryBuilder {
    pub id: String,

    pub name: String,
    pub summary: String,

    pub rating: Rating,
    pub state: State,

    pub chapters: u32,
    pub words: u32,

    pub authors: Vec<Author>,
    pub origins: Vec<Origin>,

    pub warnings: Vec<Warning>,
    pub pairings: Vec<Pairing>,
    pub characters: Vec<Character>,
    pub tags: Vec<Tag>,
}

impl StoryBuilder {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        summary: impl Into<String>,
        rating: Rating,
        state: State,
        chapters: u32,
        words: u32,
    ) -> Self {
        Self {
            id: id.into(),

            name: name.into(),
            summary: summary.into(),

            rating,
            state,

            chapters,
            words,

            authors: vec![],
            origins: vec![],

            warnings: vec![],
            pairings: vec![],
            characters: vec![],
            tags: vec![],
        }
    }

    pub fn finish(self) -> Story {
        Story {
            id: self.id,

            name: self.name,
            summary: self.summary,

            square: Square {
                rating: self.rating,
                state: self.state,
                warnings: !self.warnings.is_empty(),
            },

            chapters: self.chapters,
            words: self.words,

            authors: self.authors,
            origins: self.origins,

            warnings: self.warnings,
            pairings: self.pairings,
            characters: self.characters,
            tags: self.tags,

            series: None,

            created: Utc.ymd(2020, 6, 8).and_hms(7, 22, 3),
            updated: Utc.ymd(2020, 6, 8).and_hms(7, 22, 3),
        }
    }

    pub fn with_author(mut self, id: impl Into<String>, name: impl Into<String>) -> Self {
        self.authors.push(author(id, name));

        self
    }

    pub fn with_origin(mut self, id: impl Into<String>, name: impl Into<String>) -> Self {
        self.origins.push(origin(id, name));

        self
    }

    pub fn with_warning(mut self, id: impl Into<String>, name: impl Into<String>) -> Self {
        self.warnings.push(warning(id, name));

        self
    }

    pub fn with_pairing(
        mut self,
        id: impl Into<String>,
        platonic: bool,
        build: impl FnOnce(PairingBuilder) -> PairingBuilder,
    ) -> Self {
        let PairingBuilder {
            id,
            platonic,
            characters,
        } = build(PairingBuilder {
            id: id.into(),

            platonic,

            characters: vec![],
        });

        self.pairings.push(Pairing {
            id,

            platonic,

            characters,

            created: Utc.ymd(2020, 6, 8).and_hms(7, 22, 3),
            updated: Utc.ymd(2020, 6, 8).and_hms(7, 22, 3),
        });

        self
    }

    pub fn with_character(mut self, id: impl Into<String>, name: impl Into<String>) -> Self {
        self.characters.push(character(id, name));

        self
    }

    pub fn with_tag(mut self, id: impl Into<String>, name: impl Into<String>) -> Self {
        self.tags.push(tag(id, name));

        self
    }
}
