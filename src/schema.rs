use std::fmt;

pub trait Schema {
    fn schema(m: &mut impl fmt::Write) -> fmt::Result;
}
