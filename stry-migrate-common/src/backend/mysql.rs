use {
    crate::{
        backend::ToMySQL,
        models::{Column, ColumnDefault, Enum, ForeignKey, Item, Schema, Table, Types},
    },
    std::io,
};

impl<'i, W: io::Write> ToMySQL<W> for Schema<'i> {
    fn to_mysql(&self, writer: &mut W) -> anyhow::Result<()> {
        for item in &self.items {
            item.to_mysql(writer)?;

            writeln!(writer)?;
        }

        Ok(())
    }
}

impl<'i, W: io::Write> ToMySQL<W> for Item<'i> {
    fn to_mysql(&self, writer: &mut W) -> anyhow::Result<()> {
        match &self {
            Item::Enum(decl) => decl.to_mysql(writer)?,
            Item::Table(decl) => decl.to_mysql(writer)?,
        }

        Ok(())
    }
}

impl<'i, W: io::Write> ToMySQL<W> for Enum<'i> {
    fn to_mysql(&self, _writer: &mut W) -> anyhow::Result<()> {
        todo!()
    }
}

impl<'i, W: io::Write> ToMySQL<W> for Table<'i> {
    fn to_mysql(&self, _writer: &mut W) -> anyhow::Result<()> {
        todo!()
    }
}

impl<'i, W: io::Write> ToMySQL<W> for Column<'i> {
    fn to_mysql(&self, _writer: &mut W) -> anyhow::Result<()> {
        todo!()
    }
}

impl<'i, W: io::Write> ToMySQL<W> for Types<'i> {
    fn to_mysql(&self, _writer: &mut W) -> anyhow::Result<()> {
        todo!()
    }
}

impl<'i, W: io::Write> ToMySQL<W> for ColumnDefault<'i> {
    fn to_mysql(&self, _writer: &mut W) -> anyhow::Result<()> {
        todo!()
    }
}

impl<'i, W: io::Write> ToMySQL<W> for ForeignKey<'i> {
    fn to_mysql(&self, _writer: &mut W) -> anyhow::Result<()> {
        todo!()
    }
}
