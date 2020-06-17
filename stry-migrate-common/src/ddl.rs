use crate::models::{self, ColumnDefault, ForeignKey, Types};

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[derive(serde::Deserialize)]
pub struct Database<'i> {
    name: &'i str,

    #[serde(default = "Default::default")]
    tables: Vec<Table<'i>>,
}

impl<'i> From<Database<'i>> for models::Schema<'i> {
    fn from(old: Database<'i>) -> Self {
        let mut items = Vec::new();

        for table in old.tables {
            items.push(models::Item::Table(table.into()));
        }

        models::Schema { items }
    }
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[derive(serde::Deserialize)]
pub struct Table<'i> {
    name: &'i str,

    #[serde(default = "Default::default")]
    not_exists: bool,

    columns: Vec<Column<'i>>,

    #[serde(default = "Default::default")]
    foreign_keys: Vec<ForeignKey<'i>>,
}

impl<'i> From<Table<'i>> for models::Table<'i> {
    fn from(old: Table<'i>) -> Self {
        let mut columns = Vec::with_capacity(old.columns.len());
        let mut primary_keys = Vec::with_capacity(old.columns.len());
        let mut unique_keys = Vec::with_capacity(old.columns.len());

        for column in old.columns {
            if column.primary {
                primary_keys.push(old.name);
            }

            if column.unique {
                unique_keys.push(old.name);
            }

            columns.push(column.into());
        }

        models::Table {
            name: old.name,
            not_exists: old.not_exists,
            columns,
            primary_keys,
            foreign_keys: old.foreign_keys,
            unique_keys,
        }
    }
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
#[derive(serde::Deserialize)]
pub struct Column<'i> {
    name: &'i str,

    #[serde(rename = "type")]
    typ: Types<'i>,

    #[serde(default = "Default::default")]
    required: bool,

    #[serde(default = "Default::default")]
    primary: bool,

    #[serde(default = "Default::default")]
    unique: bool,

    // TODO: use this
    #[serde(default = "Default::default")]
    size: u32,

    #[serde(default = "Default::default")]
    default: ColumnDefault<'i>,
}

impl<'i> From<Column<'i>> for models::Column<'i> {
    fn from(old: Column<'i>) -> Self {
        models::Column {
            name: old.name,
            typ: old.typ,
            not_null: old.required,
            default: old.default,
        }
    }
}
