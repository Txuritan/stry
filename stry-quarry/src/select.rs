use {
    crate::{Condition, Join, JoinClause, Order, Parameter, Predicate, Table, Value, Variant},
    fnv::FnvHashMap,
    std::fmt::{self, Write as _},
};

macro_rules! right_condition {
    ($con:expr, $buf:ident, $params:ident, $var:ident, $sym:expr) => {
        match $con {
            Some(Value::Select(selector)) => {
                let (query, params) = selector.build($var)?;

                $params.extend(params);

                write!(&mut $buf, "({})", query)?;
            }
            Some(Value::Parameter(param)) => {
                $params.push(param);

                write!(&mut $buf, concat!(" ", $sym, " ?"))?;
            }
            Some(Value::Value(value)) => write!(&mut $buf, concat!(" ", $sym, " {}"), value)?,
            None => panic!("BUG: SQL condition does not have right side value"),
        }
    };
}

#[derive(Debug)]
pub struct Select<'q> {
    tables: Vec<Table<'q>>,

    aliases: Option<FnvHashMap<&'q str, &'q str>>,
    fields: Option<Vec<&'q str>>,

    order: Option<Vec<(&'q str, Order)>>,

    joins: Option<Vec<JoinClause<'q>>>,

    groupings: Option<Vec<&'q str>>,

    havings: Option<Vec<&'q str>>,

    conditions: Option<Vec<Condition<'q>>>,

    limit: Option<Value<'q>>,
    offset: Option<Value<'q>>,

    parameters: Option<Vec<Parameter>>,
}

impl<'q> Select<'q> {
    pub fn table<T>(mut self, table: T) -> Select<'q>
    where
        T: Into<Table<'q>>,
    {
        self.tables.push(table.into());

        self
    }

    /// `AS`
    pub fn alias(mut self, table: &'q str, alias: &'q str) -> Select<'q> {
        self.aliases
            .get_or_insert_with(FnvHashMap::default)
            .insert(table, alias);

        self
    }

    pub fn field(mut self, field: &'q str) -> Select<'q> {
        self.fields
            .get_or_insert_with(|| Vec::with_capacity(1))
            .push(field);

        self
    }

    pub fn fields(mut self, fields: &[&'q str]) -> Select<'q> {
        let items = self
            .fields
            .get_or_insert_with(|| Vec::with_capacity(fields.len()));

        for field in fields {
            items.push(field);
        }

        self
    }

    /// `ORDER BY`
    pub fn order_by(mut self, stmt: &'q str, order: Order) -> Select<'q> {
        self.order
            .get_or_insert_with(|| Vec::with_capacity(1))
            .push((stmt, order));

        self
    }

    pub fn join<T>(mut self, table: T, left: &'q str, right: &'q str, kind: Join) -> Select<'q>
    where
        T: Into<Table<'q>>,
    {
        self.joins
            .get_or_insert_with(|| Vec::with_capacity(1))
            .push(JoinClause {
                table: table.into(),
                left,
                right,
                kind,
            });

        self
    }

    pub fn left_join<T>(mut self, table: T, left: &'q str, right: &'q str) -> Select<'q>
    where
        T: Into<Table<'q>>,
    {
        self.joins
            .get_or_insert_with(|| Vec::with_capacity(1))
            .push(JoinClause {
                table: table.into(),
                left,
                right,
                kind: Join::Left,
            });

        self
    }

    /// `GROUP BY`
    pub fn group_by(mut self, stmt: &'q str) -> Select<'q> {
        self.groupings
            .get_or_insert_with(|| Vec::with_capacity(1))
            .push(stmt);

        self
    }

    /// `HAVING`
    pub fn having(mut self, stmt: &'q str) -> Select<'q> {
        self.havings
            .get_or_insert_with(|| Vec::with_capacity(1))
            .push(stmt);

        self
    }

    /// `WHERE`
    pub fn filter<C>(mut self, condition: C) -> Select<'q>
    where
        C: Into<Condition<'q>>,
    {
        self.conditions
            .get_or_insert_with(|| Vec::with_capacity(1))
            .push(condition.into());

        self
    }

    /// `LIMIT`
    pub fn limit<V>(mut self, limit: V) -> Select<'q>
    where
        V: Into<Value<'q>>,
    {
        self.limit = Some(limit.into());

        self
    }

    /// `OFFSET`
    pub fn offset<V>(mut self, offset: V) -> Select<'q>
    where
        V: Into<Value<'q>>,
    {
        self.offset = Some(offset.into());

        self
    }

    pub fn build(self, variant: Variant) -> Result<(String, Vec<Parameter>), fmt::Error> {
        let mut buff = String::with_capacity(512);
        let mut parameters = Vec::new();

        write!(&mut buff, "SELECT")?;

        if let Some(fields) = self.fields {
            let len = fields.len() - 1;

            for (i, field) in fields.into_iter().enumerate() {
                write!(&mut buff, " {}", field)?;

                if i != len {
                    write!(&mut buff, ",")?;
                }
            }
        }

        for table in self.tables {
            match table {
                Table::Alias { table, alias } => {
                    write!(&mut buff, " FROM {} {}", table, alias)?;
                }
                Table::Name(table) => {
                    write!(&mut buff, " FROM {}", table)?;
                }
            }
        }

        if let Some(joins) = self.joins {
            for join in joins {
                match join.kind {
                    Join::Inner => write!(&mut buff, " INNER JOIN")?,
                    Join::Left => write!(&mut buff, " LEFT JOIN")?,
                }

                match join.table {
                    Table::Alias { table, alias } => {
                        write!(&mut buff, " {} {}", table, alias)?;
                    }
                    Table::Name(table) => {
                        write!(&mut buff, " {}", table)?;
                    }
                }

                write!(&mut buff, " ON {} = {}", join.left, join.right)?;
            }
        }

        if let Some(conditions) = self.conditions {
            write!(&mut buff, " WHERE")?;

            let len = conditions.len() - 1;

            for (i, condition) in conditions.into_iter().enumerate() {
                write!(&mut buff, " {}", condition.left)?;

                match condition.predicate {
                    Predicate::Eq => {
                        right_condition!(condition.right, buff, parameters, variant, "=");
                    }
                    Predicate::NotEq => {
                        right_condition!(condition.right, buff, parameters, variant, "<>");
                    }
                    Predicate::Gt => {
                        right_condition!(condition.right, buff, parameters, variant, ">");
                    }
                    Predicate::GtEq => {
                        right_condition!(condition.right, buff, parameters, variant, ">=");
                    }
                    Predicate::Lt => {
                        right_condition!(condition.right, buff, parameters, variant, "<");
                    }
                    Predicate::LtEq => {
                        right_condition!(condition.right, buff, parameters, variant, "<=");
                    }
                    Predicate::In => match condition.right {
                        Some(Value::Select(_)) => todo!("TODO: Allow `IN` to be a sub-query"),
                        Some(Value::Parameter(param)) => {
                            parameters.push(param);

                            write!(&mut buff, " IN (?)",)?;
                        }
                        Some(Value::Value(value)) => write!(&mut buff, " IN ({})", value)?,
                        None => panic!("BUG: SQL condition does not have right side value"),
                    },
                    Predicate::Bt => match condition.right {
                        Some(Value::Select(_)) => todo!("TODO: Allow `BETWEEN` to be a sub-query"),
                        Some(Value::Parameter(param)) => {
                            parameters.push(param);

                            write!(&mut buff, " BETWEEN ? AND ?",)?;
                        }
                        Some(Value::Value(value)) => write!(&mut buff, " BETWEEN {}", value)?,
                        None => panic!("BUG: SQL condition does not have right side value"),
                    },
                    Predicate::Like => {
                        right_condition!(condition.right, buff, parameters, variant, "LIKE");
                    }
                    Predicate::Null => {
                        write!(&mut buff, " IS NULL")?;
                    }
                    Predicate::NotNull => {
                        write!(&mut buff, " IS NOT NULL")?;
                    }
                }

                if i != len {
                    write!(&mut buff, " AND")?;
                }
            }
        }

        if let Some(groupings) = self.groupings {
            for group in groupings {
                write!(&mut buff, " GROUP BY {}", group)?;
            }
        }

        if let Some(orders) = self.order {
            write!(&mut buff, " ORDER BY")?;

            let len = orders.len() - 1;

            for (i, (stmt, order)) in orders.into_iter().enumerate() {
                write!(&mut buff, " {} {}", stmt, order)?;

                if i != len {
                    write!(&mut buff, ",")?;
                }
            }
        }

        if let Some(limit) = self.limit {
            match limit {
                Value::Select(_) => todo!("TODO: Allow `LIMIT` to be a sub-query"),
                Value::Parameter(param) => {
                    parameters.push(param);

                    write!(&mut buff, " LIMIT ?")?;
                }
                Value::Value(value) => write!(&mut buff, " LIMIT {}", value)?,
            }
        }

        if let Some(offset) = self.offset {
            match offset {
                Value::Select(_) => todo!("TODO: Allow `OFFSET` to be a sub-query"),
                Value::Parameter(param) => {
                    parameters.push(param);

                    write!(&mut buff, " OFFSET ?")?;
                }
                Value::Value(value) => write!(&mut buff, " OFFSET {}", value)?,
            }
        }

        write!(&mut buff, ";")?;

        buff.shrink_to_fit();

        Ok((buff, parameters))
    }
}

#[allow(clippy::needless_lifetimes)]
pub fn select<'q, T>(table: T) -> Select<'q>
where
    T: Into<Table<'q>>,
{
    Select {
        tables: vec![table.into()],

        aliases: None,
        fields: None,
        order: None,
        joins: None,
        conditions: None,
        groupings: None,
        havings: None,

        limit: None,
        offset: None,

        parameters: None,
    }
}

#[cfg(test)]
mod test_super {
    use crate::prelude::*;

    #[test]
    fn test() {
        let query = select("StoryAuthor".alias("SA"))
            .field("SA.StoryId")
            .left_join("Story".alias("S"), "S.Id", "SA.StoryId")
            .filter("SA.AuthorId".is_eq("blank".to_string()))
            .order_by("S.Updated", Desc)
            .limit(10)
            .offset(0);

        let (query, parameters) = query.build(SQLite).unwrap();

        assert_eq!(
            "SELECT SA.StoryId FROM StoryAuthor SA LEFT JOIN Story S ON S.Id = SA.StoryId WHERE SA.AuthorId = ? ORDER BY S.Updated DESC LIMIT ? OFFSET ?;",
            query,
        );

        assert_eq!(
            vec![
                Parameter::String("blank".into()),
                Parameter::Signed32(10),
                Parameter::Signed32(0),
            ],
            parameters,
        );
    }
}
