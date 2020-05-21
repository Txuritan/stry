use {
    clap::{App, Arg, SubCommand},
    stry_migrate::commands::ddl,
};

fn main() -> anyhow::Result<()> {
    let matches = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .subcommand(
            SubCommand::with_name("ddl")
                .help("For working with database RON files")
                .subcommand(
                    SubCommand::with_name("generate")
                        .help("Takes a database RON file and turns it into SQL")
                        .arg(
                            Arg::with_name("input")
                                .help("The database schema RON file")
                                .required(true)
                                .takes_value(true)
                                .value_name("FILE"),
                        )
                        .arg(
                            Arg::with_name("output")
                                .help("The file to write the database schema to")
                                .required(true)
                                .takes_value(true)
                                .value_name("OUTPUT"),
                        )
                        .arg(
                            Arg::with_name("style")
                                .short("s")
                                .long("style")
                                .help("The SQL style to output the database schema as")
                                .takes_value(true)
                                .value_name("STYLE")
                                .possible_values(&["postgres", "sqlite"])
                                .default_value("sqlite"),
                        ),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("ddl", Some(ddl_matches)) => match ddl_matches.subcommand() {
            ("generate", Some(generate_matches)) => {
                match (
                    generate_matches.value_of("input"),
                    generate_matches.value_of("output"),
                    generate_matches.value_of("style"),
                ) {
                    (Some(file), Some(output), Some(style)) => {
                        ddl::generate(file, output, style)?;
                    }
                    _ => {
                        println!("{}", generate_matches.usage());
                    }
                }
            }
            _ => {
                println!("{}", ddl_matches.usage());
            }
        },
        _ => {
            println!("{}", matches.usage());
        }
    }

    Ok(())
}
