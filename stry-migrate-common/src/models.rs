use std::{fmt, str::FromStr};

#[derive(Clone, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Schema<'i> {
    pub items: Vec<Item<'i>>,
}

#[derive(Clone, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum Item<'i> {
    Enum(Enum<'i>),
    Table(Table<'i>),
}

#[derive(Clone, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Enum<'i> {
    pub name: &'i str,
    pub variants: Vec<&'i str>,
}

#[derive(Clone, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Table<'i> {
    pub name: &'i str,
    pub not_exists: bool,
    pub columns: Vec<Column<'i>>,
    pub primary_keys: Vec<&'i str>,
    pub foreign_keys: Vec<ForeignKey<'i>>,
    pub unique_keys: Vec<&'i str>,
}

#[derive(Clone, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Column<'i> {
    pub name: &'i str,
    pub typ: Types<'i>,
    pub not_null: bool,
    pub default: ColumnDefault<'i>,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[derive(serde::Deserialize)]
pub enum Types<'i> {
    Boolean,

    // Text
    Char,
    Varchar,
    Text,

    // Numbers
    Number,
    SmallInt,
    MediumInt,
    BigInt,
    Int,
    Serial,

    // Floats
    Float,
    Real,
    Numeric,
    Decimal,

    // Date/Time
    DateTime,

    Raw(&'i str),
}

impl<'i> FromStr for Types<'i> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bigInt" => Ok(Types::BigInt),
            "bool" | "boolean" => Ok(Types::Boolean),
            "char" => Ok(Types::Char),
            "dateTime" => Ok(Types::DateTime),
            "decimal" => Ok(Types::Decimal),
            "float" => Ok(Types::Float),
            "int" => Ok(Types::Int),
            "mediumInt" => Ok(Types::MediumInt),
            "number" => Ok(Types::Number),
            "numeric" => Ok(Types::Numeric),
            "real" => Ok(Types::Real),
            "serial" => Ok(Types::Serial),
            "smallInt" => Ok(Types::SmallInt),
            "text" => Ok(Types::Text),
            "varchar" => Ok(Types::Varchar),
            t => anyhow::bail!("`{}` is not a valid column type", t),
        }
    }
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[derive(serde::Deserialize)]
pub enum ColumnDefault<'i> {
    None,
    Now,
    Null,
    Raw(&'i str),
}

impl<'i> Default for ColumnDefault<'i> {
    fn default() -> Self {
        ColumnDefault::None
    }
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[derive(serde::Deserialize)]
pub struct ForeignKey<'i> {
    pub local: &'i str,
    pub table: &'i str,
    pub foreign: &'i str,

    #[serde(default = "Default::default")]
    pub delete: Action,

    #[serde(default = "Default::default")]
    pub update: Action,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[derive(serde::Deserialize)]
pub enum Action {
    NoAction,
    Restrict,
    SetNull,
    SetDefault,
    Cascade,
}

impl Default for Action {
    fn default() -> Self {
        Action::NoAction
    }
}

impl FromStr for Action {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "no action" => Ok(Action::NoAction),
            "restrict" => Ok(Action::Restrict),
            "set null" => Ok(Action::SetNull),
            "set default" => Ok(Action::SetDefault),
            "cascade" => Ok(Action::Cascade),
            t => anyhow::bail!("`{}` is not a valid action", t),
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Action::NoAction => "NO ACTION",
                Action::Restrict => "RESTRICT",
                Action::SetNull => "SET NULL",
                Action::SetDefault => "SET DEFAULT",
                Action::Cascade => "CASCADE",
            }
        )?;

        Ok(())
    }
}
