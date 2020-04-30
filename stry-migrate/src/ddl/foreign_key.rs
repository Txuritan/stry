use {crate::ddl::ToSql, std::io};

#[derive(serde::Deserialize, Debug)]
pub struct ForeignKey {
    table: String,
    local: String,
    foreign: String,
}

// TODO: turn into a function
// MySQL:       FOREIGN KEY (local) REFERENCES table(foreign)
// SQLite:      FOREIGN KEY (local) REFERENCES table(foreign)
// PostgreSQL:  FOREIGN KEY (local) REFERENCES table(foreign)
impl<W: io::Write> ToSql<W> for ForeignKey {
    fn to_mysql(&self, writer: &mut W, is_last: bool) -> io::Result<()> {
        writeln!(
            writer,
            "  FOREIGN KEY ({}) REFERENCES {}({}){}",
            self.local,
            self.table,
            self.foreign,
            if is_last { "" } else { "," },
        )?;

        Ok(())
    }

    fn to_postgresql(&self, writer: &mut W, is_last: bool) -> io::Result<()> {
        writeln!(
            writer,
            "  FOREIGN KEY ({}) REFERENCES {}({}){}",
            self.local,
            self.table,
            self.foreign,
            if is_last { "" } else { "," },
        )?;

        Ok(())
    }

    fn to_sqlite(&self, writer: &mut W, is_last: bool) -> io::Result<()> {
        writeln!(
            writer,
            "  FOREIGN KEY ({}) REFERENCES {}({}){}",
            self.local,
            self.table,
            self.foreign,
            if is_last { "" } else { "," },
        )?;

        Ok(())
    }
}
