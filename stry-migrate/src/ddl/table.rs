// TODO: Clean up table writer
// TODO: Add PRIMARY KEY, ON {DELETE, UPDATE} support to MySQL and PostgreSQL

use {
    crate::ddl::{self, Column, DatabaseType, ToSql},
    std::io,
};

#[derive(serde::Deserialize, Debug)]
pub enum Action {
    NoAction,
    Restrict,
    SetNull,
    SetDefault,
    Cascade,
}

impl Action {
    fn as_str(&self) -> &'static str {
        match self {
            Action::NoAction => "NO ACTION",
            Action::Restrict => "RESTRICT",
            Action::SetNull => "SET NULL",
            Action::SetDefault => "SET DEFAULT",
            Action::Cascade => "CASCADE",
        }
    }
}

impl Default for Action {
    fn default() -> Self {
        Action::NoAction
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct ForeignKey {
    table: String,

    local: String,

    foreign: String,

    #[serde(default = "Default::default")]
    delete: Action,

    #[serde(default = "Default::default")]
    update: Action,
}

#[derive(serde::Deserialize, Debug)]
pub struct Table {
    name: String,

    #[serde(default = "Default::default")]
    not_exists: bool,

    columns: Vec<Column>,

    #[serde(default = "Default::default")]
    foreign_keys: Vec<ForeignKey>,

    #[serde(skip)]
    primary_columns: Option<Vec<String>>,
}

impl Table {
    fn is_last(is_none: bool, is_empty: bool) -> bool {
        match (is_none, !is_empty) {
            (true, false) => true,
            (true, true) => false,
            (false, true) => false,
            (false, false) => false,
        }
    }

    fn check_primary<'c>(columns: &'c [Column]) -> Option<Vec<&'c str>> {
        let mut primaries: Option<Vec<&'c str>> = None;

        for column in columns {
            if column.primary {
                let primary = primaries.get_or_insert(Vec::with_capacity(columns.len()));

                primary.push(&column.name);
            }
        }

        primaries
    }

    fn to_mysql(&self, writer: &mut impl io::Write, typ: DatabaseType) -> anyhow::Result<()> {
        write!(writer, "CREATE TABLE")?;

        if self.not_exists {
            write!(writer, " IF NOT EXISTS")?;
        }

        write!(writer, " {} (", self.name)?;

        writeln!(writer)?;

        let primary_columns = Self::check_primary(&self.columns);

        let mut columns_peekable = self.columns.iter().peekable();

        while let Some(column) = columns_peekable.next() {
            column.to_sql(
                writer,
                typ,
                ddl::column::Params {
                    multiple_primary: primary_columns.is_some(),
                    is_last: Self::is_last(
                        columns_peekable.peek().is_none(),
                        self.foreign_keys.is_empty(),
                    ),
                },
            )?;
        }

        let mut foreign_keys_peekable = self.foreign_keys.iter().peekable();

        while let Some(foreign_key) = foreign_keys_peekable.next() {
            writeln!(
                writer,
                "  FOREIGN KEY ({}) REFERENCES {}({}){}",
                foreign_key.local,
                foreign_key.table,
                foreign_key.foreign,
                if foreign_keys_peekable.peek().is_none() {
                    ""
                } else {
                    ","
                },
            )?;
        }

        writeln!(writer, ") ENGINE=InnoDB;",)?;

        Ok(())
    }

    fn to_postgresql(&self, writer: &mut impl io::Write, typ: DatabaseType) -> anyhow::Result<()> {
        write!(writer, "CREATE TABLE")?;

        if self.not_exists {
            write!(writer, " IF NOT EXISTS")?;
        }

        write!(writer, " {} (", self.name)?;

        writeln!(writer)?;

        let primary_columns = Self::check_primary(&self.columns);

        let mut columns_peekable = self.columns.iter().peekable();

        while let Some(column) = columns_peekable.next() {
            column.to_sql(
                writer,
                typ,
                ddl::column::Params {
                    multiple_primary: primary_columns.is_some(),
                    is_last: Self::is_last(
                        columns_peekable.peek().is_none(),
                        self.foreign_keys.is_empty(),
                    ),
                },
            )?;
        }

        let mut foreign_keys_peekable = self.foreign_keys.iter().peekable();

        while let Some(foreign_key) = foreign_keys_peekable.next() {
            writeln!(
                writer,
                "  FOREIGN KEY ({}) REFERENCES {}({}){}",
                foreign_key.local,
                foreign_key.table,
                foreign_key.foreign,
                if foreign_keys_peekable.peek().is_none() {
                    ""
                } else {
                    ","
                },
            )?;
        }

        writeln!(writer, ");",)?;

        Ok(())
    }

    fn to_sqlite(&self, writer: &mut impl io::Write, typ: DatabaseType) -> anyhow::Result<()> {
        write!(writer, "CREATE TABLE")?;

        if self.not_exists {
            write!(writer, " IF NOT EXISTS")?;
        }

        write!(writer, " {} (", self.name)?;

        writeln!(writer)?;

        let primary_columns = Self::check_primary(&self.columns);

        let mut columns_peekable = self.columns.iter().peekable();

        while let Some(column) = columns_peekable.next() {
            column.to_sql(
                writer,
                typ,
                ddl::column::Params {
                    multiple_primary: primary_columns
                        .as_ref()
                        .map(|m| m.len() > 1)
                        .unwrap_or_else(|| false),
                    is_last: Self::is_last(
                        columns_peekable.peek().is_none(),
                        self.foreign_keys.is_empty(),
                    ),
                },
            )?;
        }

        let mut foreign_keys_peekable = self.foreign_keys.iter().peekable();

        while let Some(foreign_key) = foreign_keys_peekable.next() {
            write!(
                writer,
                "  FOREIGN KEY ({}) REFERENCES {}({}) ON UPDATE {} ON DELETE {}",
                foreign_key.local,
                foreign_key.table,
                foreign_key.foreign,
                foreign_key.update.as_str(),
                foreign_key.delete.as_str(),
            )?;

            if foreign_keys_peekable.peek().is_some()
                || primary_columns
                    .as_ref()
                    .map(|m| m.len() > 1)
                    .unwrap_or_else(|| false)
            {
                write!(writer, ",")?;
            }

            writeln!(writer)?
        }

        if let Some(primaries) = primary_columns {
            if primaries.len() != 1 {
                write!(writer, "  PRIMARY KEY (")?;

                let mut primaries_peekable = primaries.iter().peekable();

                while let Some(primary) = primaries_peekable.next() {
                    write!(writer, "{}", primary)?;

                    if primaries_peekable.peek().is_some() {
                        write!(writer, ", ")?;
                    }
                }

                write!(writer, ")")?;

                writeln!(writer)?
            }
        }

        writeln!(writer, ");",)?;

        Ok(())
    }
}

impl<W: io::Write> ToSql<W> for Table {
    type Params = ();

    fn to_sql(
        &self,
        writer: &mut W,
        typ: DatabaseType,
        _params: Self::Params,
    ) -> anyhow::Result<()> {
        match typ {
            DatabaseType::MySQL => self.to_mysql(writer, typ)?,
            DatabaseType::PostgreSQL => self.to_postgresql(writer, typ)?,
            DatabaseType::SQLite => self.to_sqlite(writer, typ)?,
        }

        Ok(())
    }
}
