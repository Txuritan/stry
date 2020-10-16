use {
    rusqlite::{functions::FunctionFlags, Connection},
    std::io::Read,
};

fn main() -> anyhow::Result<()> {
    let mut conn = Connection::open("./stry3-test-compress.db")?;

    add_compression_functions(&mut conn)?;

    let rows = {
        let mut stmt = conn.prepare("SELECT Id, Main FROM Chapter;")?;

        let rows = stmt
            .query_map(rusqlite::NO_PARAMS, |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?
            .collect::<Vec<_>>();

        let mut mapped = Vec::with_capacity(rows.len());

        for row in rows {
            mapped.push(row?);
        }

        std::mem::drop(stmt);

        mapped
    };

    for row in rows {
        let trans = conn.transaction()?;

        trans.execute(
            "UPDATE Chapter SET Main = compress(?) WHERE Id = ?;",
            rusqlite::params![row.1, row.0],
        )?;

        trans.commit()?;
    }

    conn.execute("VACUUM", rusqlite::NO_PARAMS)?;

    Ok(())
}

pub fn add_compression_functions(conn: &mut Connection) -> rusqlite::Result<()> {
    conn.create_scalar_function(
        "compress",
        1,
        FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC,
        move |ctx| {
            assert_eq!(ctx.len(), 1, "called with unexpected number of arguments");

            let text = ctx.get::<String>(0)?;
            let mut bytes = text.as_bytes();

            let mut compressor = brotli::CompressorReader::new(&mut bytes, 4096, 6, 20);

            let mut compressed = Vec::new();

            compressor
                .read_to_end(&mut compressed)
                .map_err(|err| rusqlite::Error::UserFunctionError(err.into()))?;

            Ok(compressed)
        },
    )?;

    conn.create_scalar_function(
        "decompress",
        1,
        FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC,
        move |ctx| {
            assert_eq!(ctx.len(), 1, "called with unexpected number of arguments");

            let bytes = ctx.get::<Vec<u8>>(0)?;

            let mut decompressor = brotli::Decompressor::new(&bytes[..], 4096);

            let mut decompressed = Vec::new();

            decompressor
                .read_to_end(&mut decompressed)
                .map_err(|err| rusqlite::Error::UserFunctionError(err.into()))?;

            let utf8 = String::from_utf8(decompressed)
                .map_err(|err| rusqlite::Error::UserFunctionError(err.into()))?;

            Ok(utf8)
        },
    )?;

    Ok(())
}
