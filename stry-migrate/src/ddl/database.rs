use {
    crate::ddl::{Table, ToSql},
    std::io,
};

#[derive(serde::Deserialize, Debug)]
pub struct Database {
    name: String,
    tables: Vec<Table>,
}

impl<W: io::Write> ToSql<W> for Database {
    fn to_mysql(&self, writer: &mut W, _is_last: bool) -> io::Result<()> {
        Ok(())
    }

    fn to_postgresql(&self, writer: &mut W, _is_last: bool) -> io::Result<()> {
        Ok(())
    }

    fn to_sqlite(&self, writer: &mut W, _is_last: bool) -> io::Result<()> {
        writeln!(writer, "BEGIN;")?;
        writeln!(writer)?;

        for table in self.tables.iter() {
            table.to_sqlite(writer, false)?;

            writeln!(writer)?;
        }

        writeln!(writer, "COMMIT;")?;

        Ok(())
    }
}
