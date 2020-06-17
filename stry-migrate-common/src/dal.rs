use {
    crate::models::{self, Action, Types},
    pest::{iterators::Pair, Parser},
    std::str::FromStr,
};

macro_rules! action {
    ($inner:ident, $delete:ident, $update:ident) => {
        match $inner.next() {
            Some(action_delete) if action_delete.as_rule() == Rule::ref_action_delete => {
                let mut action_delete_inner = action_delete.into_inner();

                match action_delete_inner.next() {
                    Some(action) if action.as_rule() == Rule::action => {
                        $delete = Action::from_str(action.as_str())?;
                    }
                    Some(_) => anyhow::bail!("Modifier declaration has an invalid action"),
                    None => anyhow::bail!("Modifier declaration has no action"),
                }
            }
            Some(action_update) if action_update.as_rule() == Rule::ref_action_update => {
                let mut action_update_inner = action_update.into_inner();

                match action_update_inner.next() {
                    Some(action) if action.as_rule() == Rule::action => {
                        $update = Action::from_str(action.as_str())?;
                    }
                    Some(_) => anyhow::bail!("Modifier declaration has an invalid action"),
                    None => anyhow::bail!("Modifier declaration has no action"),
                }
            }
            Some(_) => anyhow::bail!("Modifier declaration has an invalid ref_action"),
            None => {}
        }
    };
}

#[derive(pest_derive::Parser)]
#[grammar = "../dal.pest"]
pub struct DalParser;

impl DalParser {
    #[allow(clippy::wrong_self_convention)]
    pub fn into_schema<'p>(input: &'p str) -> anyhow::Result<Schema<'p>> {
        let mut pairs = Self::parse(Rule::schema, input)?;

        let pair: Pair<'p, Rule> = match pairs.next() {
            Some(pair) => pair,
            None => anyhow::bail!("Missing table or enum declaration"),
        };

        let mut items = Vec::new();

        for decl in pair.into_inner() {
            match decl.as_rule() {
                Rule::decl_enum => {
                    let value = Self::handle_enum(decl)?;

                    items.push(value);
                }
                Rule::decl_table => {
                    let value = Self::handle_table(decl)?;

                    items.push(value);
                }
                Rule::comment | Rule::EOI => continue,
                rule => todo!("not a valid decl: {:?}", rule),
            }
        }

        Ok(Schema { items })
    }

    fn handle_enum<'p>(pair: Pair<'p, Rule>) -> anyhow::Result<Decl<'p>> {
        let mut inner = pair.into_inner();

        let name = match inner.next() {
            Some(pair) if pair.as_rule() == Rule::ident => pair.as_str(),
            Some(_) => anyhow::bail!("Enum declaration has an invalid name"),
            None => anyhow::bail!("Enum declaration has no name"),
        };

        let mut variants = Vec::new();

        for pair in inner {
            if pair.as_rule() == Rule::variant {
                variants.push(pair.as_str());
            } else {
                anyhow::bail!("Enum declaration has an invalid variant");
            }
        }

        Ok(Decl::Enum { name, variants })
    }

    fn handle_table<'p>(pair: Pair<'p, Rule>) -> anyhow::Result<Decl<'p>> {
        let mut inner = pair.into_inner();

        let name = match inner.next() {
            Some(pair) if pair.as_rule() == Rule::ident => pair.as_str(),
            Some(_) => anyhow::bail!("Table declaration is missing a name"),
            None => anyhow::bail!("Table declaration is missing a name"),
        };

        let mut rows = Vec::new();

        for row in inner {
            match row.as_rule() {
                Rule::row => {
                    let value = Self::handle_row(row)?;

                    rows.push(value);
                }
                Rule::comment => continue,
                _ => anyhow::bail!("Table declaration must only have rows"),
            }
        }

        Ok(Decl::Table { name, rows })
    }

    fn handle_row<'p>(pair: Pair<'p, Rule>) -> anyhow::Result<Row<'p>> {
        let mut inner = pair.into_inner();

        let name = match inner.next() {
            Some(pair) if pair.as_rule() == Rule::ident => pair.as_str(),
            Some(_) => anyhow::bail!("Row declaration is missing a name"),
            None => anyhow::bail!("Row declaration is missing a name"),
        };

        let (null, typ) = match inner.next() {
            Some(pair) if pair.as_rule() == Rule::row_type => {
                (false, Types::from_str(pair.as_str())?)
            }
            Some(pair) if pair.as_rule() == Rule::row_type_null => {
                let mut pair_inner = pair.into_inner();

                let next = match pair_inner.next() {
                    Some(next) => next,
                    None => anyhow::bail!("Row declaration is missing a type"),
                };

                let typ = Types::from_str(next.as_str())?;

                (true, typ)
            }
            Some(_) => anyhow::bail!("Row declaration is missing a type"),
            None => anyhow::bail!("Row declaration is missing a type"),
        };

        let modifiers = match inner.next() {
            Some(modifier_pair) if modifier_pair.as_rule() == Rule::modifiers => {
                let mut modifiers = Vec::new();

                for modifier in modifier_pair.into_inner() {
                    match modifier.as_rule() {
                        Rule::modifier_default
                        | Rule::modifier_primary
                        | Rule::modifier_ref
                        | Rule::modifier_unique => {
                            let value = Self::handle_modifier(modifier)?;

                            modifiers.push(value);
                        }
                        _ => anyhow::bail!("Row declaration has an invalid modifier"),
                    }
                }

                modifiers
            }
            Some(_) => anyhow::bail!("ROw declaration has an invalid modifier list"),
            None => vec![],
        };

        Ok(Row {
            name,
            typ,
            null,
            modifiers,
        })
    }

    fn handle_modifier<'p>(pair: Pair<'p, Rule>) -> anyhow::Result<Modifier<'p>> {
        match pair.as_rule() {
            Rule::modifier_default => {
                let mut inner = pair.into_inner();

                let next = match inner.next() {
                    Some(next) if next.as_rule() == Rule::modifier_default_value => next,
                    Some(_) => anyhow::bail!("Modifier declaration has an invalid default value"),
                    None => anyhow::bail!("Modifier declaration has no default value"),
                };

                Ok(match next.as_str() {
                    "now()" => Modifier::DefaultDateTime,
                    "null" => Modifier::DefaultNull,
                    value => Modifier::Default { value },
                })
            }
            Rule::modifier_primary => Ok(Modifier::PrimaryKey),
            Rule::modifier_ref => {
                let mut inner = pair.into_inner();

                let table = match inner.next() {
                    Some(next) if next.as_rule() == Rule::ident => next,
                    Some(_) => {
                        anyhow::bail!("Modifier declaration has an invalid reference table value");
                    }
                    None => anyhow::bail!("Modifier declaration has no reference table value"),
                };

                let column = match inner.next() {
                    Some(next) if next.as_rule() == Rule::ident => next,
                    Some(_) => {
                        anyhow::bail!("Modifier declaration has an invalid reference column value");
                    }
                    None => anyhow::bail!("Modifier declaration has no reference column value"),
                };

                let (delete, update) = match inner.next() {
                    Some(ref_action) if ref_action.as_rule() == Rule::ref_action => {
                        Self::handle_actions(ref_action)?
                    }
                    Some(_) => {
                        anyhow::bail!("Modifier declaration has an invalid reference action");
                    }
                    None => (Action::default(), Action::default()),
                };

                Ok(Modifier::Reference {
                    table: table.as_str(),
                    column: column.as_str(),
                    delete,
                    update,
                })
            }
            Rule::modifier_unique => Ok(Modifier::Unique),
            _ => unreachable!(),
        }
    }

    fn handle_actions(pair: Pair<'_, Rule>) -> anyhow::Result<(Action, Action)> {
        let mut ref_inner = pair.into_inner();

        let mut delete = Action::default();
        let mut update = Action::default();

        action!(ref_inner, delete, update);
        action!(ref_inner, delete, update);

        Ok((delete, update))
    }
}

#[derive(Debug, PartialEq)]
pub struct Schema<'p> {
    pub items: Vec<Decl<'p>>,
}

impl<'p> From<Schema<'p>> for models::Schema<'p> {
    fn from(old: Schema<'p>) -> Self {
        let mut items = Vec::new();

        for item in old.items {
            items.push(item.into());
        }

        models::Schema { items }
    }
}

#[derive(Debug, PartialEq)]
pub enum Decl<'p> {
    Enum {
        name: &'p str,
        variants: Vec<&'p str>,
    },
    Table {
        name: &'p str,
        rows: Vec<Row<'p>>,
    },
}

impl<'p> From<Decl<'p>> for models::Item<'p> {
    fn from(old: Decl<'p>) -> Self {
        match old {
            Decl::Enum { name, variants } => models::Item::Enum(models::Enum { name, variants }),
            Decl::Table { name, rows } => {
                let mut columns = Vec::new();
                let mut primary_keys = Vec::new();
                let mut foreign_keys = Vec::new();
                let mut unique_keys = Vec::new();

                for row in rows {
                    let mut default = None;

                    for modifier in &row.modifiers {
                        match modifier {
                            Modifier::Default { value } => {
                                default = Some(models::ColumnDefault::Raw(*value))
                            }
                            Modifier::DefaultDateTime => default = Some(models::ColumnDefault::Now),
                            Modifier::DefaultNull => default = Some(models::ColumnDefault::Null),
                            Modifier::PrimaryKey => primary_keys.push(row.name),
                            Modifier::Reference {
                                table,
                                column,
                                delete,
                                update,
                            } => foreign_keys.push(models::ForeignKey {
                                local: row.name,
                                table: (*table),
                                foreign: (*column),
                                delete: delete.clone(),
                                update: update.clone(),
                            }),
                            Modifier::Unique => unique_keys.push(row.name),
                        }
                    }

                    columns.push(models::Column {
                        name: row.name,
                        typ: row.typ,
                        not_null: !row.null,
                        default: default.unwrap_or_else(models::ColumnDefault::default),
                    });
                }

                models::Item::Table(models::Table {
                    name,
                    not_exists: true, // TODO: figure out a syntax for this
                    columns,
                    primary_keys,
                    foreign_keys,
                    unique_keys,
                })
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Row<'p> {
    pub name: &'p str,
    pub typ: Types<'p>,
    pub null: bool,
    pub modifiers: Vec<Modifier<'p>>,
}

#[derive(Debug, PartialEq)]
pub enum Modifier<'p> {
    Default {
        value: &'p str,
    },
    DefaultDateTime,
    DefaultNull,
    PrimaryKey,
    Reference {
        table: &'p str,
        column: &'p str,
        delete: Action,
        update: Action,
    },
    Unique,
}

#[cfg(test)]
mod tests {
    pub use {super::*, crate::models::Types};

    mod enums {
        use super::*;

        const ENUM: &str = "enum Rating {
            Explicit
            Mature
            Teen
            General
        }";

        #[test]
        fn simple() {
            assert_eq!(
                Schema {
                    items: vec![Decl::Enum {
                        name: "Rating",
                        variants: vec!["Explicit", "Mature", "Teen", "General"],
                    }]
                },
                DalParser::into_schema(ENUM).unwrap()
            )
        }
    }

    mod tables {
        use super::*;

        const TABLE: &str = "table Settings {
            key text [primary key]
            value text
            created dateTime [default: now()]
            updated dateTime [default: now()]
        }";

        const TABLE_NULL: &str = "table Settings {
            key text [primary key]
            value text?
            created dateTime [default: now()]
            updated dateTime [default: now()]
        }";

        const TABLE_REFERENCE: &str = "table Settings {
            key text [primary key]
            otherOne text [ref: Other.id (delete: cascade, update: cascade)]
            otherTwo text [ref: Other.id (delete: cascade)]
            otherThree text [ref: Other.id (update: cascade)]
            created dateTime [default: now()]
            updated dateTime [default: now()]
        }";

        fn def_table(row: Row) -> Schema {
            Schema {
                items: vec![Decl::Table {
                    name: "Settings",
                    rows: vec![
                        Row {
                            name: "key",
                            typ: Types::Text,
                            null: false,
                            modifiers: vec![Modifier::PrimaryKey],
                        },
                        row,
                        Row {
                            name: "created",
                            typ: Types::DateTime,
                            null: false,
                            modifiers: vec![Modifier::DefaultDateTime],
                        },
                        Row {
                            name: "updated",
                            typ: Types::DateTime,
                            null: false,
                            modifiers: vec![Modifier::DefaultDateTime],
                        },
                    ],
                }],
            }
        }

        #[test]
        fn simple() {
            assert_eq!(
                def_table(Row {
                    name: "value",
                    typ: Types::Text,
                    null: false,
                    modifiers: vec![],
                }),
                DalParser::into_schema(TABLE).unwrap()
            )
        }

        #[test]
        fn simple_null() {
            assert_eq!(
                def_table(Row {
                    name: "value",
                    typ: Types::Text,
                    null: true,
                    modifiers: vec![],
                }),
                DalParser::into_schema(TABLE_NULL).unwrap()
            )
        }

        #[test]
        fn reference() {
            assert_eq!(
                Schema {
                    items: vec![Decl::Table {
                        name: "Settings",
                        rows: vec![
                            Row {
                                name: "key",
                                typ: Types::Text,
                                null: false,
                                modifiers: vec![Modifier::PrimaryKey],
                            },
                            Row {
                                name: "otherOne",
                                typ: Types::Text,
                                null: false,
                                modifiers: vec![Modifier::Reference {
                                    table: "Other",
                                    column: "id",
                                    delete: Action::Cascade,
                                    update: Action::Cascade,
                                }],
                            },
                            Row {
                                name: "otherTwo",
                                typ: Types::Text,
                                null: false,
                                modifiers: vec![Modifier::Reference {
                                    table: "Other",
                                    column: "id",
                                    delete: Action::Cascade,
                                    update: Action::default(),
                                }],
                            },
                            Row {
                                name: "otherThree",
                                typ: Types::Text,
                                null: false,
                                modifiers: vec![Modifier::Reference {
                                    table: "Other",
                                    column: "id",
                                    delete: Action::default(),
                                    update: Action::Cascade,
                                }],
                            },
                            Row {
                                name: "created",
                                typ: Types::DateTime,
                                null: false,
                                modifiers: vec![Modifier::DefaultDateTime],
                            },
                            Row {
                                name: "updated",
                                typ: Types::DateTime,
                                null: false,
                                modifiers: vec![Modifier::DefaultDateTime],
                            },
                        ],
                    }],
                },
                DalParser::into_schema(TABLE_REFERENCE).unwrap()
            )
        }
    }
}
