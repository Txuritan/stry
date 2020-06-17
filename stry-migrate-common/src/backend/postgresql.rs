use {
    crate::{
        backend::ToPostgreSQL,
        models::{Column, ColumnDefault, Enum, ForeignKey, Item, Schema, Table, Types},
    },
    std::io,
};

impl<'i, W: io::Write> ToPostgreSQL<W> for Schema<'i> {
    fn to_postgresql(&self, writer: &mut W) -> anyhow::Result<()> {
        for item in &self.items {
            item.to_postgresql(writer)?;

            writeln!(writer)?;
        }

        Ok(())
    }
}

impl<'i, W: io::Write> ToPostgreSQL<W> for Item<'i> {
    fn to_postgresql(&self, writer: &mut W) -> anyhow::Result<()> {
        match &self {
            Item::Enum(decl) => decl.to_postgresql(writer)?,
            Item::Table(decl) => decl.to_postgresql(writer)?,
        }

        Ok(())
    }
}

impl<'i, W: io::Write> ToPostgreSQL<W> for Enum<'i> {
    fn to_postgresql(&self, _writer: &mut W) -> anyhow::Result<()> {
        todo!()
    }
}

impl<'i, W: io::Write> ToPostgreSQL<W> for Table<'i> {
    fn to_postgresql(&self, _writer: &mut W) -> anyhow::Result<()> {
        todo!()
    }
}

impl<'i, W: io::Write> ToPostgreSQL<W> for Column<'i> {
    fn to_postgresql(&self, _writer: &mut W) -> anyhow::Result<()> {
        todo!()
    }
}

impl<'i, W: io::Write> ToPostgreSQL<W> for Types<'i> {
    fn to_postgresql(&self, _writer: &mut W) -> anyhow::Result<()> {
        todo!()
    }
}

impl<'i, W: io::Write> ToPostgreSQL<W> for ColumnDefault<'i> {
    fn to_postgresql(&self, _writer: &mut W) -> anyhow::Result<()> {
        todo!()
    }
}

impl<'i, W: io::Write> ToPostgreSQL<W> for ForeignKey<'i> {
    fn to_postgresql(&self, _writer: &mut W) -> anyhow::Result<()> {
        todo!()
    }
}
