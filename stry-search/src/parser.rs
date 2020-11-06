use {
    pest::{iterators::Pair, Parser},
    std::{borrow::Cow, iter::IntoIterator, ops::Deref},
    stry_models::Rating,
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
#[grammar = "search.pest"]
pub struct SearchParser;

impl SearchParser {
    pub fn parse_to_structure<'p>(input: &'p str) -> anyhow::Result<Values<'p>> {
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

        Ok(Values { values: search })
    }

    fn handle_category<'p>(pair: Pair<'p, Rule>) -> anyhow::Result<Value<'p>> {
        let mut inner = pair.into_inner();

        let (include, inner_pair) = match (inner.next(), inner.next()) {
            (Some(_), Some(inner_pair)) => (false, inner_pair),
            (Some(inner_pair), None) => (true, inner_pair),
            _ => anyhow::bail!("Invalid category type"),
        };

        let value = match inner_pair.as_rule() {
            Rule::value => Value::General(include, inner_pair.as_str().into()),
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

    fn handle_friends<'p>(include: bool, pair: Pair<'p, Rule>) -> anyhow::Result<Value<'p>> {
        let mut characters = Vec::new();

        for inner_pair in pair.into_inner() {
            if inner_pair.as_rule() == Rule::value {
                characters.push(Value::Character(include, inner_pair.as_str().into()));
            } else {
                anyhow::bail!("pairing type must be value or character");
            }
        }

        Ok(Value::Friends(include, characters))
    }

    fn handle_pairing<'p>(include: bool, pair: Pair<'p, Rule>) -> anyhow::Result<Value<'p>> {
        let mut characters = Vec::new();

        for inner_pair in pair.into_inner() {
            if inner_pair.as_rule() == Rule::value {
                characters.push(Value::Character(include, inner_pair.as_str().into()));
            } else {
                anyhow::bail!("pairing type must be value or character");
            }
        }

        Ok(Value::Pairing(include, characters))
    }

    fn handle_character<'p>(include: bool, pair: Pair<'p, Rule>) -> anyhow::Result<Value<'p>> {
        simple!([include, pair], "character", Rule::value, Value::Character)
    }

    fn handle_fandom<'p>(include: bool, pair: Pair<'p, Rule>) -> anyhow::Result<Value<'p>> {
        simple!([include, pair], "fandom", Rule::value, Value::Fandom)
    }

    fn handle_rating<'p>(include: bool, pair: Pair<'p, Rule>) -> anyhow::Result<Value<'p>> {
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

                    Ok(Value::Rating(include, rating))
                } else {
                    anyhow::bail!("Not a valid rating, rating inner is not rating_inner");
                }
            }
            None => {
                anyhow::bail!("Not a valid rating, rating inner has no pairs");
            }
        }
    }

    fn handle_tag<'p>(include: bool, pair: Pair<'p, Rule>) -> anyhow::Result<Value<'p>> {
        simple!([include, pair], "tag", Rule::value, Value::General)
    }
}

pub struct Values<'p> {
    values: Vec<Value<'p>>,
}

impl<'p> Deref for Values<'p> {
    type Target = Vec<Value<'p>>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl<'p> IntoIterator for Values<'p> {
    type Item = Value<'p>;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

#[derive(Debug)]
pub enum Value<'p> {
    Friends(bool, Vec<Value<'p>>),
    Pairing(bool, Vec<Value<'p>>),
    Character(bool, Cow<'p, str>),
    Fandom(bool, Cow<'p, str>),
    General(bool, Cow<'p, str>),
    Rating(bool, Rating),
}

impl<'p> Value<'p> {
    pub fn is_included(&self) -> bool {
        match self {
            Value::Friends(included, _) => *included,
            Value::Pairing(included, _) => *included,
            Value::Character(included, _) => *included,
            Value::Fandom(included, _) => *included,
            Value::General(included, _) => *included,
            Value::Rating(included, _) => *included,
        }
    }
}
