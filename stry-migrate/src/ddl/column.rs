use {crate::ddl::ToSql, std::io};

pub enum Schema {
    MySQL,
    PostgreSQL,
    SQLite,
}

#[derive(serde::Deserialize, Debug)]
pub enum Types {
    Boolean,
    // Text
    String,
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

    Raw(String),
}

impl Types {
    fn name(&self, schema: Schema) -> &str {
        match schema {
            Schema::MySQL => match self {
                Types::Boolean => "BOOLEAN",
                Types::Char => "CHAR",
                Types::Varchar => "VARCHAR",
                Types::String | Types::Text => "TEXT",
                Types::SmallInt => "SMALLINT",
                Types::MediumInt => "MEDIUMINT",
                Types::BigInt => "BIGINT",
                Types::Number | Types::Int => "INT",
                Types::Serial => "SERIAL",
                Types::Float => "FLOAT",
                Types::Real => "REAL",
                Types::Numeric | Types::Decimal => "NUMERIC",
                Types::DateTime => "INTEGER",
                Types::Raw(raw) => raw.as_str(),
            },
            Schema::PostgreSQL => match self {
                Types::Boolean => "BOOLEAN",
                Types::Char => "CHAR",
                Types::Varchar => "VARCHAR",
                Types::String | Types::Text => "TEXT",
                Types::SmallInt => "SMALLINT",
                Types::BigInt => "BIGINT",
                Types::Number | Types::Int | Types::MediumInt => "INTEGER",
                Types::Serial => "SERIAL",
                Types::Float => "FLOAT",
                Types::Real => "REAL",
                Types::Decimal => "DECIMAL",
                Types::Numeric => "NUMERIC",
                Types::DateTime => "TIMESTAMP WITHOUT TIME ZONE",
                Types::Raw(raw) => raw.as_str(),
            },
            Schema::SQLite => match self {
                Types::String | Types::Char | Types::Varchar | Types::Text => "TEXT",
                Types::Number | Types::SmallInt | Types::MediumInt | Types::BigInt | Types::Int | Types::Serial => {
                    "INTEGER"
                }
                Types::Float | Types::Real | Types::Numeric | Types::Decimal => "REAL",
                Types::Boolean | Types::DateTime => "NUMERIC",
                Types::Raw(raw) => raw.as_str(),
            },
        }
    }
}

#[derive(serde::Deserialize, Debug, PartialEq)]
enum TypeDefault {
    None,
    Now,
    Null,
    Raw(String),
}

impl Default for TypeDefault {
    fn default() -> Self {
        TypeDefault::None
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct Column {
    name: String,

    #[serde(rename = "type")]
    typ: Types,

    #[serde(default = "Default::default")]
    required: bool,

    #[serde(default = "Default::default")]
    primary: bool,

    #[serde(default = "Default::default")]
    size: u32,

    #[serde(default = "Default::default")]
    default: TypeDefault,
}

impl<W: io::Write> ToSql<W> for Column {
    fn to_mysql(&self, writer: &mut W, _is_last: bool) -> io::Result<()> {
        Ok(())
    }

    fn to_postgresql(&self, writer: &mut W, _is_last: bool) -> io::Result<()> {
        Ok(())
    }

    fn to_sqlite(&self, writer: &mut W, is_last: bool) -> io::Result<()> {
        write!(writer, "    {} {}",self.name, self.typ.name(Schema::SQLite))?;

        if self.required {
            write!(writer, " NOT NULL")?;
        }

        if self.primary {
            write!(writer, " PRIMARY KEY")?;
        }

        if self.default != TypeDefault::None {
            write!(writer, " DEFAULT")?;

            match &self.default {
                TypeDefault::Now => {
                    write!(writer, " (DATETIME('now', 'utc'))")?;
                },
                TypeDefault::Null => {
                    write!(writer, " NULL")?;
                },
                TypeDefault::Raw(raw) => {
                    write!(writer, " {}", raw)?;
                }
                TypeDefault::None => unreachable!(),
            }
        }

        if !is_last {
            write!(writer, ",")?;
        }

        writeln!(writer)?;

        Ok(())
    }
}
