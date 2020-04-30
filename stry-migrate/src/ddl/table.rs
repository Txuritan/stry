use {
    crate::ddl::{Column, ForeignKey, ToSql},
    std::io,
};

#[derive(serde::Deserialize, Debug)]
pub struct Table {
    name: String,
    #[serde(default = "Default::default")]
    not_exists: bool,
    columns: Vec<Column>,
    #[serde(default = "Default::default")]
    foreign_keys: Vec<ForeignKey>,
}

impl<W: io::Write> ToSql<W> for Table {
    fn to_mysql(&self, writer: &mut W, _is_last: bool) -> io::Result<()> {
        writeln!(
            writer,
            "CREATE TABLE{} `{}` (",
            if self.not_exists {
                " IF NOT EXISTS"
            } else {
                ""
            },
            self.name,
        )?;

        let mut columns_peekable = self.columns.iter().peekable();
        while let Some(column) = columns_peekable.next() {
            column.to_mysql(
                writer,
                match (
                    columns_peekable.peek().is_none(),
                    !self.foreign_keys.is_empty(),
                ) {
                    (true, false) => true,
                    (true, true) => false,
                    (false, true) => false,
                    (false, false) => false,
                },
            )?;
        }

        let mut foreign_keys_peekable = self.foreign_keys.iter().peekable();
        while let Some(foreign_key) = foreign_keys_peekable.next() {
            foreign_key.to_mysql(writer, foreign_keys_peekable.peek().is_none())?;
        }

        writeln!(writer, ") ENGINE=InnoDB;",)?;

        Ok(())
    }

    fn to_postgresql(&self, writer: &mut W, _is_last: bool) -> io::Result<()> {
        writeln!(
            writer,
            "CREATE TABLE{} {} (",
            if self.not_exists {
                " IF NOT EXISTS"
            } else {
                ""
            },
            self.name,
        )?;

        let mut columns_peekable = self.columns.iter().peekable();
        while let Some(column) = columns_peekable.next() {
            column.to_sqlite(
                writer,
                match (
                    columns_peekable.peek().is_none(),
                    !self.foreign_keys.is_empty(),
                ) {
                    (true, false) => true,
                    (true, true) => false,
                    (false, true) => false,
                    (false, false) => false,
                },
            )?;
        }

        let mut foreign_keys_peekable = self.foreign_keys.iter().peekable();
        while let Some(foreign_key) = foreign_keys_peekable.next() {
            foreign_key.to_sqlite(writer, foreign_keys_peekable.peek().is_none())?;
        }

        writeln!(writer, ");",)?;

        Ok(())
    }

    fn to_sqlite(&self, writer: &mut W, _is_last: bool) -> io::Result<()> {
        writeln!(
            writer,
            "CREATE TABLE{} {} (",
            if self.not_exists {
                " IF NOT EXISTS"
            } else {
                ""
            },
            self.name,
        )?;

        let mut columns_peekable = self.columns.iter().peekable();
        while let Some(column) = columns_peekable.next() {
            column.to_sqlite(
                writer,
                match (
                    columns_peekable.peek().is_none(),
                    !self.foreign_keys.is_empty(),
                ) {
                    (true, false) => true,
                    (true, true) => false,
                    (false, true) => false,
                    (false, false) => false,
                },
            )?;
        }

        let mut foreign_keys_peekable = self.foreign_keys.iter().peekable();
        while let Some(foreign_key) = foreign_keys_peekable.next() {
            foreign_key.to_sqlite(writer, foreign_keys_peekable.peek().is_none())?;
        }

        writeln!(writer, ");",)?;

        Ok(())
    }
}
