pub mod commands;

use {
    crate::commands::{dal, ddl},
    clap::{App, Arg, SubCommand},
};

fn main() -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        // .json()
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    tracing_log::LogTracer::init()?;

    let matches = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .subcommand(
            SubCommand::with_name("dal")
                // .help("For working with database DAL files")
                .subcommand(
                    SubCommand::with_name("generate")
                        // .help("Takes a database DAL file and turns it into SQL")
                        .arg(
                            Arg::with_name("input")
                                .help("The database schema DAL file")
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
                                .possible_values(&["mysql", "postgres", "sqlite"])
                                .default_value("sqlite"),
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("ddl")
                // .help("For working with database RON files")
                .subcommand(
                    SubCommand::with_name("generate")
                        // .help("Takes a database RON file and turns it into SQL")
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
                                .possible_values(&["mysql", "postgres", "sqlite"])
                                .default_value("sqlite"),
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("nanoid").arg(
                Arg::with_name("count")
                    .help("Amount of IDs to generate")
                    .required(true)
                    .takes_value(true)
                    .value_name("COUNT")
                    .default_value("16"),
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
        ("dal", Some(ddl_matches)) => match ddl_matches.subcommand() {
            ("generate", Some(generate_matches)) => {
                match (
                    generate_matches.value_of("input"),
                    generate_matches.value_of("output"),
                    generate_matches.value_of("style"),
                ) {
                    (Some(file), Some(output), Some(style)) => {
                        dal::generate(file, output, style)?;
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
        ("nanoid", Some(nanoid_matches)) => match nanoid_matches.value_of("count") {
            Some(count) => {
                let amount: u32 = count.parse()?;

                let div = amount / 4;
                let rem = amount % 4;

                for _ in 0..=div {
                    println!(
                        "{}    {}    {}    {}",
                        stry_common::nanoid::nanoid(),
                        stry_common::nanoid::nanoid(),
                        stry_common::nanoid::nanoid(),
                        stry_common::nanoid::nanoid()
                    );
                }

                match rem {
                    0 => {}
                    1 => println!("{}", stry_common::nanoid::nanoid()),
                    2 => println!(
                        "{}    {}",
                        stry_common::nanoid::nanoid(),
                        stry_common::nanoid::nanoid()
                    ),
                    3 => println!(
                        "{}    {}    {}",
                        stry_common::nanoid::nanoid(),
                        stry_common::nanoid::nanoid(),
                        stry_common::nanoid::nanoid()
                    ),
                    _ => unreachable!(),
                }
            }
            None => {
                println!("{}", nanoid_matches.usage());
            }
        },
        _ => {
            println!("{}", matches.usage());
        }
    }

    Ok(())
}
