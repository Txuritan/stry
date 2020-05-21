use {
    crate::ddl::{DatabaseType, Table, ToSql},
    std::{fs, io, path::Path},
};

#[derive(serde::Deserialize, Debug)]
pub struct Database {
    name: String,

    #[serde(default = "Default::default")]
    include: Option<Vec<String>>,

    #[serde(default = "Default::default")]
    tables: Vec<Table>,
}

impl Database {
    pub fn read_from(file: impl AsRef<Path>) -> anyhow::Result<Self> {
        let file_buff = fs::read(file)?;
        let mut database: Database = ron::de::from_bytes(&file_buff)?;

        if let Some(includes) = &database.include {
            for include in includes {
                let included = Database::read_from(include)?;

                database.tables.extend(included.tables);
            }
        }

        Ok(database)
    }

    fn to_mysql(&self, _writer: &mut impl io::Write, _typ: DatabaseType) -> anyhow::Result<()> {
        Ok(())
    }

    fn to_postgresql(
        &self,
        _writer: &mut impl io::Write,
        _typ: DatabaseType,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn to_sqlite(&self, writer: &mut impl io::Write, typ: DatabaseType) -> anyhow::Result<()> {
        writeln!(writer, "BEGIN;")?;
        writeln!(writer)?;

        for table in self.tables.iter() {
            table.to_sql(writer, typ, ())?;

            writeln!(writer)?;
        }

        writeln!(writer, "COMMIT;")?;

        Ok(())
    }
}

impl<W: io::Write> ToSql<W> for Database {
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
