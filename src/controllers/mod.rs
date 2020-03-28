pub mod author;
pub mod chapter;
pub mod origin;
pub mod story;
pub mod tag;

#[derive(Debug, db_derive::Table)]
#[table(exists, schema)]
pub struct Counter {
    #[table(rename = "Count")]
    pub count: i64,
}

#[derive(Debug, db_derive::Table)]
#[table(exists, schema)]
pub struct Id {
    #[table(rename = "Id")]
    pub id: String,
}
