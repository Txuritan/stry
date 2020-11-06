// TODO:
//   - SQL functions for values

mod select;

use std::fmt;

pub use crate::select::{select, Select};

pub mod prelude {
    pub use crate::{
        select, Alias as _, ConditionPredicate as _,
        Order::{Asc, Desc},
        Parameter, Value,
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
                V: Into<Value<'q>>;
        )*
    };

    (impl; [ $( $fn_name:ident => $predicate:expr , )* ]) => {
        $(
            fn $fn_name<V>(self, right: V) -> Condition<'q>
            where
                V: Into<Value<'q>>,
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
    right: Option<Value<'q>>,
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
        write!(
            f,
            "{}",
            match self {
                Order::Asc => "ASC",
                Order::Desc => "DESC",
            }
        )
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
pub enum Value<'q> {
    Select(Box<Select<'q>>),
    Value(&'static str),
    Parameter(Parameter),
}

impl<'q> From<Select<'q>> for Value<'q> {
    fn from(value: Select<'q>) -> Self {
        Value::Select(Box::new(value))
    }
}

impl<'q> From<&'static str> for Value<'q> {
    fn from(value: &'static str) -> Self {
        Value::Value(value)
    }
}

impl<'q> From<Parameter> for Value<'q> {
    fn from(value: Parameter) -> Self {
        Value::Parameter(value)
    }
}

#[derive(Debug, PartialEq)]
pub enum Parameter {
    Signed32(i32),
    Signed64(i64),
    Float64(f64),
    String(String),
    Bytes(Vec<u8>),

    // Custom `stry` types
    #[cfg(feature = "stry-types")]
    Rating(stry_models::Rating),
}

macro_rules! impl_param {
    ($( $typ:ty => $pa:path , )*) => {
        $(
            impl From<$typ> for Parameter {
                fn from(value: $typ) -> Self {
                    $pa(value)
                }
            }

            impl<'q> From<$typ> for Value<'q> {
                fn from(value: $typ) -> Self {
                    Value::Parameter($pa(value))
                }
            }
        )*
    };
}

impl_param! {
    i32 => Parameter::Signed32,
    i64 => Parameter::Signed64,
    f64 => Parameter::Float64,
    String => Parameter::String,
    Vec<u8> => Parameter::Bytes,
}

#[cfg(feature = "stry-types")]
impl_param! {
    stry_models::Rating => Parameter::Rating,
}
