use {
    std::{fs, io::BufWriter, str::FromStr},
    stry_migrate_common::{
        backend::{DatabaseType, ToSql},
        ddl::Database,
        models::Schema,
    },
};

pub fn generate(file: &str, output: &str, style: &str) -> anyhow::Result<()> {
    let file_buff = fs::read(file)?;
    let database: Database = ron::de::from_bytes(&file_buff)?;

    let schema: Schema = database.into();

    let mut buff = BufWriter::new(Vec::new());

    schema.to_sql(&mut buff, DatabaseType::from_str(style)?)?;

    fs::write(output, &buff.into_inner()?)?;

    Ok(())
}
