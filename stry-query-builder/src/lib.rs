use {fnv::FnvHashMap, std::fmt::{self, Write as _}};


pub mod prelude {
    pub use crate::{
        select, Alias as _, ConditionPredicate as _,
        Order::{Asc, Desc},
        Value::Parameter,
        Variant::{PostgreSQL, SQLite},
    };
}

#[derive(Clone, Copy, Debug)]
pub enum Variant {
    PostgreSQL,
    SQLite,
}

pub trait Alias<'q> {
    fn alias(self, alias: &'q str) -> (Self, &'q str)
    where
        Self: Sized;
}

impl<'q> Alias<'q> for &'q str {
    fn alias(self, alias: &'q str) -> (Self, &'q str)
    where
        Self: Sized,
    {
        (self, alias)
    }
}

macro_rules! impl_predicates {
    (trait; [ $( $fn_name:ident , )* ]) => {
        $(
            fn $fn_name<V>(self, right: V) -> Condition<'q>
            where
                V: Into<Value<&'q str>>;
        )*
    };

    (impl; [ $( $fn_name:ident => $predicate:expr , )* ]) => {
        $(
            fn $fn_name<V>(self, right: V) -> Condition<'q>
            where
                V: Into<Value<&'q str>>,
            {
                Condition {
                    left: self,
                    predicate: $predicate,
                    right: Some(right.into()),
                }
            }
        )*
    };
}

pub trait ConditionPredicate<'q> {
    impl_predicates!(trait; [
        is_eq,
        is_not_eq,
        is_gt,
        is_gt_eq,
        is_lt,
        is_lt_eq,
        is_in,
        is_bt,
        is_like,
    ]);

    fn is_null(self) -> Condition<'q>;

    fn is_not_null(self) -> Condition<'q>;
}

impl<'q> ConditionPredicate<'q> for &'q str {
    impl_predicates!(impl; [
        is_eq => Predicate::Eq,
        is_not_eq => Predicate::NotEq,
        is_gt => Predicate::Gt,
        is_gt_eq => Predicate::GtEq,
        is_lt => Predicate::Lt,
        is_lt_eq => Predicate::LtEq,
        is_in => Predicate::In,
        is_bt => Predicate::Bt,
        is_like => Predicate::Like,
    ]);

    fn is_null(self) -> Condition<'q> {
        Condition {
            left: self,
            predicate: Predicate::Null,
            right: None,
        }
    }

    fn is_not_null(self) -> Condition<'q> {
        Condition {
            left: self,
            predicate: Predicate::NotNull,
            right: None,
        }
    }
}

#[derive(Debug)]
pub enum Join {
    Inner,
    Left,
}

#[derive(Debug)]
pub struct Condition<'q> {
    left: &'q str,
    predicate: Predicate,
    right: Option<Value<&'q str>>,
}

#[derive(Debug)]
pub struct JoinClause<'q> {
    table: Table<'q>,

    left: &'q str,
    right: &'q str,

    kind: Join,
}

#[derive(Debug)]
pub enum Order {
    Asc,
    Desc,
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Order::Asc => "ASC",
            Order::Desc => "DESC",
        })
    }
}

#[derive(Debug)]
pub enum Predicate {
    Eq,
    NotEq,
    Gt,
    GtEq,
    Lt,
    LtEq,
    In,
    Bt,
    Like,
    Null,
    NotNull,
}

#[derive(Debug)]
pub enum Table<'q> {
    Alias { table: &'q str, alias: &'q str },
    Name(&'q str),
}

impl<'q> From<&'q str> for Table<'q> {
    fn from(value: &'q str) -> Self {
        Table::Name(value)
    }
}

impl<'q> From<(&'q str, &'q str)> for Table<'q> {
    fn from((table, alias): (&'q str, &'q str)) -> Self {
        Table::Alias { table, alias }
    }
}

#[derive(Debug)]
pub enum Value<T> {
    Value(T),
    Parameter,
}

impl<T> From<T> for Value<T> {
    fn from(value: T) -> Self {
        Value::Value(value)
    }
}

#[derive(Debug)]
pub struct Select<'q> {
    table: Table<'q>,

    aliases: Option<FnvHashMap<&'q str, &'q str>>,
    fields: Option<Vec<&'q str>>,

    order: Option<Vec<(&'q str, Order)>>,

    joins: Option<Vec<JoinClause<'q>>>,

    groupings: Option<Vec<&'q str>>,

    havings: Option<Vec<&'q str>>,

    conditions: Option<Vec<Condition<'q>>>,

    limit: Option<Value<usize>>,
    offset: Option<Value<usize>>,
}

impl<'q> Select<'q> {
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
        V: Into<Value<usize>>,
    {
        self.limit = Some(limit.into());

        self
    }

    /// `OFFSET`
    pub fn offset<V>(mut self, offset: V) -> Select<'q>
    where
        V: Into<Value<usize>>,
    {
        self.offset = Some(offset.into());

        self
    }

    pub fn build(self, variant: Variant) -> Result<String, fmt::Error> {
        let mut buff = String::with_capacity(512);

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

        match self.table {
            Table::Alias { table, alias } => {
                write!(&mut buff, " FROM {} {}", table, alias)?;
            }
            Table::Name(table) => {
                write!(&mut buff, " FROM {}", table)?;
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
                        write!(&mut buff, " = {}", match condition.right {
                            Some(Value::Parameter) => "?",
                            Some(Value::Value(value)) => value,
                            None => panic!("BUG: SQL condition does not have right side value"),
                        })?;
                    }
                    Predicate::NotEq => {
                        write!(&mut buff, " <> {}", match condition.right {
                            Some(Value::Parameter) => "?",
                            Some(Value::Value(value)) => value,
                            None => panic!("BUG: SQL condition does not have right side value"),
                        })?;
                    }
                    Predicate::Gt => {
                        write!(&mut buff, " > {}", match condition.right {
                            Some(Value::Parameter) => "?",
                            Some(Value::Value(value)) => value,
                            None => panic!("BUG: SQL condition does not have right side value"),
                        })?;
                    }
                    Predicate::GtEq => {
                        write!(&mut buff, " >= {}", match condition.right {
                            Some(Value::Parameter) => "?",
                            Some(Value::Value(value)) => value,
                            None => panic!("BUG: SQL condition does not have right side value"),
                        })?;
                    }
                    Predicate::Lt => {
                        write!(&mut buff, " < {}", match condition.right {
                            Some(Value::Parameter) => "?",
                            Some(Value::Value(value)) => value,
                            None => panic!("BUG: SQL condition does not have right side value"),
                        })?;
                    }
                    Predicate::LtEq => {
                        write!(&mut buff, " <= {}", match condition.right {
                            Some(Value::Parameter) => "?",
                            Some(Value::Value(value)) => value,
                            None => panic!("BUG: SQL condition does not have right side value"),
                        })?;
                    }
                    Predicate::In => {
                        write!(&mut buff, " IN ({})", match condition.right {
                            Some(Value::Parameter) => "?",
                            Some(Value::Value(value)) => value,
                            None => panic!("BUG: SQL condition does not have right side value"),
                        })?;
                    }
                    Predicate::Bt => {
                        write!(&mut buff, " BETWEEN {}", match condition.right {
                            Some(Value::Parameter) => "? AND ?",
                            Some(Value::Value(value)) => value,
                            None => panic!("BUG: SQL condition does not have right side value"),
                        })?;
                    }
                    Predicate::Like => {
                        write!(&mut buff, " LIKE {}", match condition.right {
                            Some(Value::Parameter) => "?",
                            Some(Value::Value(value)) => value,
                            None => panic!("BUG: SQL condition does not have right side value"),
                        })?;
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
                Value::Parameter => write!(&mut buff, " LIMIT ?")?,
                Value::Value(value) => write!(&mut buff, " LIMIT {}", value)?,
            }
        }

        if let Some(offset) = self.offset {
            match offset {
                Value::Parameter => write!(&mut buff, " OFFSET ?")?,
                Value::Value(value) => write!(&mut buff, " OFFSET {}", value)?,
            }
        }

        write!(&mut buff, ";")?;

        buff.shrink_to_fit();

        Ok(buff)
    }
}

#[allow(clippy::needless_lifetimes)]
pub fn select<'q, T>(table: T) -> Select<'q>
where
    T: Into<Table<'q>>,
{
    Select {
        table: table.into(),

        aliases: None,
        fields: None,
        order: None,
        joins: None,
        conditions: None,
        groupings: None,
        havings: None,

        limit: None,
        offset: None,
    }
}

#[cfg(test)]
mod test_super {
    use super::prelude::*;

    #[test]
    fn test() {
        let query = select("StoryAuthor".alias("SA"))
            .field("SA.StoryId")
            .left_join("Story".alias("S"), "S.Id", "SA.StoryId")
            .filter("SA.AuthorId".is_eq(Parameter))
            .order_by("S.Updated", Desc)
            .limit(Parameter)
            .offset(Parameter);

        assert_eq!(
            "SELECT SA.StoryId FROM StoryAuthor SA LEFT JOIN Story S ON S.Id = SA.StoryId WHERE SA.AuthorId = ? ORDER BY S.Updated DESC LIMIT ? OFFSET ?;",
            query.build(SQLite).unwrap()
        );
    }
}
