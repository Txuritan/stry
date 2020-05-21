use {
    crate::ddl::{DatabaseType, ToSql},
    std::io,
};

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
    fn name(&self, typ: DatabaseType) -> &str {
        match typ {
            DatabaseType::MySQL => match self {
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
            DatabaseType::PostgreSQL => match self {
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
            DatabaseType::SQLite => match self {
                Types::String | Types::Char | Types::Text => "TEXT",
                Types::Varchar => "VARCHAR",
                Types::Number | Types::SmallInt | Types::MediumInt | Types::Int | Types::Serial => {
                    "INTEGER"
                }
                Types::BigInt => "BIGINT",
                Types::Float | Types::Real | Types::Numeric => "REAL",
                Types::Decimal => "DECIMAL",
                Types::DateTime => "DATETIME",
                Types::Boolean => "BOOLEAN",
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

pub struct Params {
    pub(crate) multiple_primary: bool,
    pub(crate) is_last: bool,
}

#[derive(serde::Deserialize, Debug)]
pub struct Column {
    pub(crate) name: String,

    #[serde(rename = "type")]
    typ: Types,

    #[serde(default = "Default::default")]
    required: bool,

    #[serde(default = "Default::default")]
    pub(crate) primary: bool,

    #[serde(default = "Default::default")]
    size: u32,

    #[serde(default = "Default::default")]
    default: TypeDefault,
}

impl Column {
    fn to_mysql(
        &self,
        _writer: &mut impl io::Write,
        _typ: DatabaseType,
        _params: Params,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn to_postgresql(
        &self,
        _writer: &mut impl io::Write,
        _typ: DatabaseType,
        _params: Params,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn to_sqlite(
        &self,
        writer: &mut impl io::Write,
        typ: DatabaseType,
        params: Params,
    ) -> anyhow::Result<()> {
        write!(writer, "  {} {}", self.name, self.typ.name(typ))?;

        if self.required || self.primary {
            write!(writer, " NOT NULL")?;
        }

        if self.primary && !params.multiple_primary {
            write!(writer, " PRIMARY KEY")?;
        }

        if self.default != TypeDefault::None {
            write!(writer, " DEFAULT")?;

            match &self.default {
                TypeDefault::Now => {
                    write!(writer, " (DATETIME('now', 'utc'))")?;
                }
                TypeDefault::Null => {
                    write!(writer, " NULL")?;
                }
                TypeDefault::Raw(raw) => {
                    write!(writer, " {}", raw)?;
                }
                TypeDefault::None => unreachable!(),
            }
        }

        if !params.is_last || params.multiple_primary {
            write!(writer, ",")?;
        }

        writeln!(writer)?;

        Ok(())
    }
}

impl<W: io::Write> ToSql<W> for Column {
    type Params = Params;

    fn to_sql(
        &self,
        writer: &mut W,
        typ: DatabaseType,
        params: Self::Params,
    ) -> anyhow::Result<()> {
        match typ {
            DatabaseType::MySQL => self.to_mysql(writer, typ, params)?,
            DatabaseType::PostgreSQL => self.to_postgresql(writer, typ, params)?,
            DatabaseType::SQLite => self.to_sqlite(writer, typ, params)?,
        }

        Ok(())
    }
}
