use {
    crate::models::Rating,
    pest::{iterators::Pair, Parser},
    std::borrow::Cow,
};

macro_rules! simple {
    ([$include:ident, $pair:ident], $tag:literal, $rule:path, $var:path) => {{
        let mut inner_pairs = $pair.into_inner();

        match inner_pairs.next() {
            Some(inner) => {
                if inner.as_rule() == $rule {
                    Ok($var($include, inner.as_str().into()))
                } else {
                    anyhow::bail!(concat!(
                        "Not a valid ",
                        $tag,
                        ", ",
                        $tag,
                        " inner is not a value"
                    ));
                }
            }
            None => {
                anyhow::bail!(concat!(
                    "Not a valid ",
                    $tag,
                    ", ",
                    $tag,
                    " inner has no pairs"
                ));
            }
        }
    }};
}

#[derive(pest_derive::Parser)]
#[grammar = "../search.pest"]
pub struct SearchParser;

impl SearchParser {
    pub fn parse_to_structure<'p>(input: &'p str) -> anyhow::Result<Vec<SearchValue<'p>>> {
        let mut pairs = Self::parse(Rule::search, input)?;

        let mut search = Vec::new();

        match pairs.next() {
            Some(next_pair) => {
                for category in next_pair.into_inner() {
                    if category.as_rule() == Rule::category {
                        let value = Self::handle_category(category)?;

                        search.push(value);
                    } else {
                        anyhow::bail!("first rule layer must be a category");
                    }
                }
            }
            None => anyhow::bail!("there must be a starting point"),
        }

        Ok(search)
    }

    fn handle_category<'p>(pair: Pair<'p, Rule>) -> anyhow::Result<SearchValue<'p>> {
        let mut inner = pair.into_inner();

        let (include, inner_pair) = match (inner.next(), inner.next()) {
            (Some(_), Some(inner_pair)) => (false, inner_pair),
            (Some(inner_pair), None) => (true, inner_pair),
            _ => anyhow::bail!("Invalid category type"),
        };

        let value = match inner_pair.as_rule() {
            Rule::value => SearchValue::General(include, inner_pair.as_str().into()),
            Rule::friends => Self::handle_friends(include, inner_pair)?,
            Rule::pairing => Self::handle_pairing(include, inner_pair)?,
            Rule::character => Self::handle_character(include, inner_pair)?,
            Rule::tag => Self::handle_tag(include, inner_pair)?,
            Rule::fandom => Self::handle_fandom(include, inner_pair)?,
            Rule::rating => Self::handle_rating(include, inner_pair)?,
            p => anyhow::bail!("TODO: {:?}", p),
        };

        Ok(value)
    }

    fn handle_friends<'p>(include: bool, pair: Pair<'p, Rule>) -> anyhow::Result<SearchValue<'p>> {
        let mut characters = Vec::new();

        for inner_pair in pair.into_inner() {
            if inner_pair.as_rule() == Rule::value {
                characters.push(SearchValue::Character(include, inner_pair.as_str().into()));
            } else {
                anyhow::bail!("pairing type must be value or character");
            }
        }

        Ok(SearchValue::Friends(include, characters))
    }

    fn handle_pairing<'p>(include: bool, pair: Pair<'p, Rule>) -> anyhow::Result<SearchValue<'p>> {
        let mut characters = Vec::new();

        for inner_pair in pair.into_inner() {
            if inner_pair.as_rule() == Rule::value {
                characters.push(SearchValue::Character(include, inner_pair.as_str().into()));
            } else {
                anyhow::bail!("pairing type must be value or character");
            }
        }

        Ok(SearchValue::Pairing(include, characters))
    }

    fn handle_character<'p>(
        include: bool,
        pair: Pair<'p, Rule>,
    ) -> anyhow::Result<SearchValue<'p>> {
        simple!(
            [include, pair],
            "character",
            Rule::value,
            SearchValue::Character
        )
    }

    fn handle_fandom<'p>(include: bool, pair: Pair<'p, Rule>) -> anyhow::Result<SearchValue<'p>> {
        simple!([include, pair], "fandom", Rule::value, SearchValue::Fandom)
    }

    fn handle_rating<'p>(include: bool, pair: Pair<'p, Rule>) -> anyhow::Result<SearchValue<'p>> {
        let mut rating_inner_pairs = pair.into_inner();

        match rating_inner_pairs.next() {
            Some(rating_inner) => {
                if rating_inner.as_rule() == Rule::rating_inner {
                    let rating = match rating_inner.as_str() {
                        "e" | "explicit" => Rating::Explicit,
                        "m" | "mature" => Rating::Mature,
                        "t" | "teen" => Rating::Teen,
                        "g" | "general" => Rating::General,
                        _ => unreachable!(),
                    };

                    Ok(SearchValue::Rating(include, rating))
                } else {
                    anyhow::bail!("Not a valid rating, rating inner is not rating_inner");
                }
            }
            None => {
                anyhow::bail!("Not a valid rating, rating inner has no pairs");
            }
        }
    }

    fn handle_tag<'p>(include: bool, pair: Pair<'p, Rule>) -> anyhow::Result<SearchValue<'p>> {
        simple!([include, pair], "tag", Rule::value, SearchValue::General)
    }
}

#[derive(Debug)]
pub enum SearchValue<'p> {
    Friends(bool, Vec<SearchValue<'p>>),
    Pairing(bool, Vec<SearchValue<'p>>),
    Character(bool, Cow<'p, str>),
    Fandom(bool, Cow<'p, str>),
    General(bool, Cow<'p, str>),
    Rating(bool, Rating),
}

impl<'p> SearchValue<'p> {
    pub fn is_included(&self) -> bool {
        match self {
            SearchValue::Friends(included, _) => *included,
            SearchValue::Pairing(included, _) => *included,
            SearchValue::Character(included, _) => *included,
            SearchValue::Fandom(included, _) => *included,
            SearchValue::General(included, _) => *included,
            SearchValue::Rating(included, _) => *included,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SearchTypes {
    Characters,
    Fandoms,
    Friends,
    General,
    Pairings,
    Rating,
}

#[derive(Debug, Default)]
struct SplitSearch<'p> {
    characters: Option<Vec<SearchValue<'p>>>,
    fandoms: Option<Vec<SearchValue<'p>>>,
    friends: Option<Vec<SearchValue<'p>>>,
    general: Option<Vec<SearchValue<'p>>>,
    pairings: Option<Vec<SearchValue<'p>>>,
    rating: Option<Vec<SearchValue<'p>>>,
}

pub trait VecSearchExt<'p> {
    fn split_on_type(self) -> Vec<(SearchTypes, Option<Vec<SearchValue<'p>>>)>;

    fn type_count(&self) -> usize;
}

impl<'p> VecSearchExt<'p> for Vec<SearchValue<'p>> {
    fn split_on_type(self) -> Vec<(SearchTypes, Option<Vec<SearchValue<'p>>>)> {
        let mut split = SplitSearch::default();

        let mut friends = 0usize;
        let mut pairings = 0usize;
        let mut characters = 0usize;
        let mut fandoms = 0usize;
        let mut general = 0usize;
        let mut rating = 0usize;

        for search in &self {
            match search {
                SearchValue::Friends(_, _) => friends += 1,
                SearchValue::Pairing(_, _) => pairings += 1,
                SearchValue::Character(_, _) => characters += 1,
                SearchValue::Fandom(_, _) => fandoms += 1,
                SearchValue::General(_, _) => general += 1,
                SearchValue::Rating(_, _) => rating += 1,
            }
        }

        for search in self {
            match search {
                search @ SearchValue::Friends(_, _) => split
                    .friends
                    .get_or_insert_with(move || Vec::with_capacity(friends))
                    .push(search),
                search @ SearchValue::Pairing(_, _) => split
                    .pairings
                    .get_or_insert_with(move || Vec::with_capacity(pairings))
                    .push(search),
                search @ SearchValue::Character(_, _) => split
                    .characters
                    .get_or_insert_with(move || Vec::with_capacity(characters))
                    .push(search),
                search @ SearchValue::Fandom(_, _) => split
                    .fandoms
                    .get_or_insert_with(move || Vec::with_capacity(fandoms))
                    .push(search),
                search @ SearchValue::General(_, _) => split
                    .general
                    .get_or_insert_with(move || Vec::with_capacity(general))
                    .push(search),
                search @ SearchValue::Rating(_, _) => split
                    .rating
                    .get_or_insert_with(move || Vec::with_capacity(rating))
                    .push(search),
            }
        }

        vec![
            (SearchTypes::Friends, split.friends),
            (SearchTypes::Pairings, split.pairings),
            (SearchTypes::Characters, split.characters),
            (SearchTypes::Fandoms, split.fandoms),
            (SearchTypes::General, split.general),
            (SearchTypes::Rating, split.rating),
        ]
    }

    fn type_count(&self) -> usize {
        let mut friends = false;
        let mut pairings = false;
        let mut characters = false;
        let mut fandoms = false;
        let mut general = false;
        let mut rating = false;

        let mut count = 0usize;

        for search in self {
            match search {
                SearchValue::Friends(_, _) if !friends => {
                    count += 1;
                    friends = true;
                }
                SearchValue::Pairing(_, _) if !pairings => {
                    count += 1;
                    pairings = true;
                }
                SearchValue::Character(_, _) if !characters => {
                    count += 1;
                    characters = true;
                }
                SearchValue::Fandom(_, _) if !fandoms => {
                    count += 1;
                    fandoms = true;
                }
                SearchValue::General(_, _) if !general => {
                    count += 1;
                    general = true;
                }
                SearchValue::Rating(_, _) if !rating => {
                    count += 1;
                    rating = true;
                }
                _ => continue,
            }
        }

        count
    }
}
