use {
    crate::{
        backend::ToSQLite,
        models::{Column, ColumnDefault, Enum, ForeignKey, Item, Schema, Table, Types},
    },
    std::io,
};

impl<'i, W: io::Write> ToSQLite<W> for Schema<'i> {
    fn to_sqlite(&self, writer: &mut W) -> anyhow::Result<()> {
        for (i, item) in self.items.iter().enumerate() {
            item.to_sqlite(writer)?;

            writeln!(writer)?;

            if i != self.items.len() - 1 {
                writeln!(writer)?;
            }
        }

        Ok(())
    }
}

impl<'i, W: io::Write> ToSQLite<W> for Item<'i> {
    fn to_sqlite(&self, writer: &mut W) -> anyhow::Result<()> {
        match &self {
            Item::Enum(decl) => decl.to_sqlite(writer)?,
            Item::Table(decl) => decl.to_sqlite(writer)?,
        }

        Ok(())
    }
}

impl<'i, W: io::Write> ToSQLite<W> for Enum<'i> {
    fn to_sqlite(&self, _writer: &mut W) -> anyhow::Result<()> {
        // TODO: maybe log a warning?
        Ok(())
    }
}

impl<'i, W: io::Write> ToSQLite<W> for Table<'i> {
    fn to_sqlite(&self, writer: &mut W) -> anyhow::Result<()> {
        write!(writer, "CREATE TABLE")?;

        if self.not_exists {
            write!(writer, " IF NOT EXISTS")?;
        }

        write!(writer, " {} (", self.name)?;

        writeln!(writer)?;

        for column in &self.columns {
            column.to_sqlite(writer)?;

            write!(writer, ",")?;

            writeln!(writer)?;
        }

        write!(writer, "  PRIMARY KEY (")?;

        for (i, primary) in self.primary_keys.iter().enumerate() {
            write!(writer, "{}", primary)?;

            if i != self.primary_keys.len() - 1 {
                write!(writer, ", ")?;
            }
        }

        write!(writer, ")")?;

        if !self.foreign_keys.is_empty() {
            write!(writer, ",")?;
            writeln!(writer)?;

            for (i, foreign_key) in self.foreign_keys.iter().enumerate() {
                foreign_key.to_sqlite(writer)?;

                if i != self.foreign_keys.len() - 1 {
                    write!(writer, ",")?;

                    writeln!(writer)?;
                }
            }

            if self.unique_keys.is_empty() {
                writeln!(writer)?;
            }
        } else if self.unique_keys.is_empty() {
            writeln!(writer)?;
        }

        if !self.unique_keys.is_empty() {
            write!(writer, ",")?;
            writeln!(writer)?;

            write!(writer, "  UNIQUE (")?;

            for (i, unique) in self.unique_keys.iter().enumerate() {
                write!(writer, "{}", unique)?;

                if i != self.unique_keys.len() - 1 {
                    write!(writer, ", ")?;
                }
            }

            write!(writer, ")")?;

            writeln!(writer)?;
        }

        write!(writer, ");")?;

        Ok(())
    }
}

impl<'i, W: io::Write> ToSQLite<W> for Column<'i> {
    fn to_sqlite(&self, writer: &mut W) -> anyhow::Result<()> {
        write!(writer, "  {} ", self.name,)?;

        self.typ.to_sqlite(writer)?;

        if self.not_null {
            write!(writer, " NOT NULL")?;
        }

        self.default.to_sqlite(writer)?;

        Ok(())
    }
}

impl<'i, W: io::Write> ToSQLite<W> for Types<'i> {
    fn to_sqlite(&self, writer: &mut W) -> anyhow::Result<()> {
        write!(
            writer,
            "{}",
            match self {
                Types::Char | Types::Text => "TEXT",
                Types::Varchar => "VARCHAR",
                Types::Number | Types::SmallInt | Types::MediumInt | Types::Int | Types::Serial => {
                    "INTEGER"
                }
                Types::BigInt => "BIGINT",
                Types::Float | Types::Real | Types::Numeric => "REAL",
                Types::Decimal => "DECIMAL",
                Types::DateTime => "DATETIME",
                Types::Boolean => "BOOLEAN",
                Types::Raw(_) => "TEXT",
            }
        )?;

        Ok(())
    }
}

impl<'i, W: io::Write> ToSQLite<W> for ColumnDefault<'i> {
    fn to_sqlite(&self, writer: &mut W) -> anyhow::Result<()> {
        if self != &ColumnDefault::None {
            write!(writer, " DEFAULT")?;

            match self {
                ColumnDefault::Now => {
                    write!(writer, " (DATETIME('now', 'utc'))")?;
                }
                ColumnDefault::Null => {
                    write!(writer, " NULL")?;
                }
                ColumnDefault::Raw(raw) => {
                    write!(writer, " {}", raw)?;
                }
                ColumnDefault::None => unreachable!(),
            }
        }

        Ok(())
    }
}

impl<'i, W: io::Write> ToSQLite<W> for ForeignKey<'i> {
    fn to_sqlite(&self, writer: &mut W) -> anyhow::Result<()> {
        write!(
            writer,
            "  FOREIGN KEY ({}) REFERENCES {}({}) ON UPDATE {} ON DELETE {}",
            self.local, self.table, self.foreign, self.update, self.delete,
        )?;

        Ok(())
    }
}

// TODO: Maybe I can clean this up
#[cfg(test)]
mod tests {
    use crate::{backend::ToSQLite, models::*};

    #[test]
    fn simple() {
        let table = Table {
            name: "Example",
            not_exists: true,
            columns: vec![
                Column {
                    name: "Id",
                    typ: Types::Text,
                    not_null: true,
                    default: ColumnDefault::None,
                },
                Column {
                    name: "Name",
                    typ: Types::Text,
                    not_null: true,
                    default: ColumnDefault::None,
                },
            ],
            primary_keys: vec!["Id"],
            foreign_keys: vec![],
            unique_keys: vec![],
        };

        let mut buff = Vec::new();

        table
            .to_sqlite(&mut buff)
            .expect("Unable to write table to buffer");

        let utf8_buff = String::from_utf8(buff).expect("Unable to convert buff into string");

        assert_eq!(
            "CREATE TABLE IF NOT EXISTS Example (
  Id TEXT NOT NULL,
  Name TEXT NOT NULL,
  PRIMARY KEY (Id)
);",
            utf8_buff.as_str()
        );
    }

    #[test]
    fn multiple_primary_keys() {
        let table = Table {
            name: "Example",
            not_exists: true,
            columns: vec![
                Column {
                    name: "Key",
                    typ: Types::Text,
                    not_null: true,
                    default: ColumnDefault::None,
                },
                Column {
                    name: "Value",
                    typ: Types::Text,
                    not_null: true,
                    default: ColumnDefault::None,
                },
            ],
            primary_keys: vec!["Key", "Value"],
            foreign_keys: vec![],
            unique_keys: vec![],
        };

        let mut buff = Vec::new();

        table
            .to_sqlite(&mut buff)
            .expect("Unable to write table to buffer");

        let utf8_buff = String::from_utf8(buff).expect("Unable to convert buff into string");

        assert_eq!(
            "CREATE TABLE IF NOT EXISTS Example (
  Key TEXT NOT NULL,
  Value TEXT NOT NULL,
  PRIMARY KEY (Key, Value)
);",
            utf8_buff.as_str()
        );
    }

    #[test]
    fn foreign_keys() {
        let table = Table {
            name: "Example",
            not_exists: true,
            columns: vec![
                Column {
                    name: "Id",
                    typ: Types::Text,
                    not_null: true,
                    default: ColumnDefault::None,
                },
                Column {
                    name: "Name",
                    typ: Types::Text,
                    not_null: true,
                    default: ColumnDefault::None,
                },
                Column {
                    name: "Other",
                    typ: Types::Text,
                    not_null: true,
                    default: ColumnDefault::None,
                },
            ],
            primary_keys: vec!["Id"],
            foreign_keys: vec![ForeignKey {
                local: "Other",
                table: "Other",
                foreign: "Id",
                delete: Action::default(),
                update: Action::default(),
            }],
            unique_keys: vec![],
        };

        let mut buff = Vec::new();

        table
            .to_sqlite(&mut buff)
            .expect("Unable to write table to buffer");

        let utf8_buff = String::from_utf8(buff).expect("Unable to convert buff into string");

        assert_eq!(
            "CREATE TABLE IF NOT EXISTS Example (
  Id TEXT NOT NULL,
  Name TEXT NOT NULL,
  Other TEXT NOT NULL,
  PRIMARY KEY (Id),
  FOREIGN KEY (Other) REFERENCES Other(Id) ON UPDATE NO ACTION ON DELETE NO ACTION
);",
            utf8_buff.as_str()
        );
    }

    #[test]
    fn unique_keys() {
        let table = Table {
            name: "Example",
            not_exists: true,
            columns: vec![
                Column {
                    name: "Id",
                    typ: Types::Text,
                    not_null: true,
                    default: ColumnDefault::None,
                },
                Column {
                    name: "Key",
                    typ: Types::Text,
                    not_null: true,
                    default: ColumnDefault::None,
                },
                Column {
                    name: "Value",
                    typ: Types::Text,
                    not_null: true,
                    default: ColumnDefault::None,
                },
            ],
            primary_keys: vec!["Id"],
            foreign_keys: vec![],
            unique_keys: vec!["Key"],
        };

        let mut buff = Vec::new();

        table
            .to_sqlite(&mut buff)
            .expect("Unable to write table to buffer");

        let utf8_buff = String::from_utf8(buff).expect("Unable to convert buff into string");

        assert_eq!(
            "CREATE TABLE IF NOT EXISTS Example (
  Id TEXT NOT NULL,
  Key TEXT NOT NULL,
  Value TEXT NOT NULL,
  PRIMARY KEY (Id),
  UNIQUE (Key)
);",
            utf8_buff.as_str()
        );
    }

    #[test]
    fn unique_keys_foreign_keys() {
        let table = Table {
            name: "Example",
            not_exists: true,
            columns: vec![
                Column {
                    name: "Id",
                    typ: Types::Text,
                    not_null: true,
                    default: ColumnDefault::None,
                },
                Column {
                    name: "Name",
                    typ: Types::Text,
                    not_null: true,
                    default: ColumnDefault::None,
                },
                Column {
                    name: "Other",
                    typ: Types::Text,
                    not_null: true,
                    default: ColumnDefault::None,
                },
            ],
            primary_keys: vec!["Id"],
            foreign_keys: vec![ForeignKey {
                local: "Other",
                table: "Other",
                foreign: "Id",
                delete: Action::default(),
                update: Action::default(),
            }],
            unique_keys: vec!["Name"],
        };

        let mut buff = Vec::new();

        table
            .to_sqlite(&mut buff)
            .expect("Unable to write table to buffer");

        let utf8_buff = String::from_utf8(buff).expect("Unable to convert buff into string");

        assert_eq!(
            "CREATE TABLE IF NOT EXISTS Example (
  Id TEXT NOT NULL,
  Name TEXT NOT NULL,
  Other TEXT NOT NULL,
  PRIMARY KEY (Id),
  FOREIGN KEY (Other) REFERENCES Other(Id) ON UPDATE NO ACTION ON DELETE NO ACTION,
  UNIQUE (Name)
);",
            utf8_buff.as_str()
        );
    }
}
