use {
    std::{fs, io::BufWriter, str::FromStr},
    stry_migrate_common::{
        backend::{DatabaseType, ToSql},
        dal::DalParser,
        models::Schema,
    },
};

pub fn generate(file: &str, output: &str, style: &str) -> anyhow::Result<()> {
    let contents = fs::read_to_string(file)?;

    let old_schema = DalParser::into_schema(&contents)?;

    let new_schema: Schema = old_schema.into();

    let mut buff = BufWriter::new(Vec::new());

    new_schema.to_sql(&mut buff, DatabaseType::from_str(style)?)?;

    fs::write(output, &buff.into_inner()?)?;

    Ok(())
}
