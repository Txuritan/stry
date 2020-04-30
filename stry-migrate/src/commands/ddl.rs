use {
    crate::ddl::{Database, ToSql},
    std::fs,
};

pub fn generate(file: &str, output: &str, style: &str) -> anyhow::Result<()> {
    let file_buff = fs::read(file)?;
    let database: Database = ron::de::from_bytes(&file_buff)?;

    let mut output_buff = Vec::new();

    match style {
        "postgres" => {
            database.to_postgresql(&mut output_buff, true)?;
        }
        "sqlite" => {
            database.to_sqlite(&mut output_buff, true)?;
        }
        _ => unreachable!(),
    }

    fs::write(output, output_buff)?;

    Ok(())
}
