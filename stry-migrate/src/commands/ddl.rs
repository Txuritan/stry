use {
    crate::ddl::{Database, DatabaseType, ToSql},
    std::fs,
};

pub fn generate(file: &str, output: &str, style: &str) -> anyhow::Result<()> {
    let database = Database::read_from(file)?;

    let mut output_buff = Vec::new();

    match style {
        "postgres" => {
            database.to_sql(&mut output_buff, DatabaseType::PostgreSQL, ())?;
        }
        "sqlite" => {
            database.to_sql(&mut output_buff, DatabaseType::SQLite, ())?;
        }
        _ => unreachable!(),
    }

    fs::write(output, output_buff)?;

    Ok(())
}
