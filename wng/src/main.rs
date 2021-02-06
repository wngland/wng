#![forbid(unsafe_code)]

use clap::{App, Arg, SubCommand};
use config::{manually::manually, reinit::reinit, setup::setup};
use create::create;
use wng_lib::*;

fn main() -> Result<()> {
    let matches = App::new("Wanager")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Wanager is a package and projects manager for C")
        .subcommand(SubCommand::with_name("setup").about("Setup wanager."))
        .subcommand(
            SubCommand::with_name("config")
                .subcommand(
                    SubCommand::with_name("reinit").about("Reinitializes wanager configuration."),
                )
                .subcommand(
                    SubCommand::with_name("edit")
                        .arg(
                            Arg::with_name("key")
                                .required(true)
                                .index(1)
                                .help("The key to edit."),
                        )
                        .arg(
                            Arg::with_name("value")
                                .required(true)
                                .index(2)
                                .help("The value to assign to the key."),
                        )
                        .about("Edits wanager configuration."),
                )
                .about("Configures wanager."),
        )
        .subcommand(
            SubCommand::with_name("new")
                .arg(
                    Arg::with_name("path")
                        .required(true)
                        .index(1)
                        .help("The directory where the project will be created"),
                )
                .about("Creates a new wanager project."),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("new") {
        create(matches.value_of("path").unwrap(), None, true)?;
    } else if let Some(matches) = matches.subcommand_matches("config") {
        if let Some(_) = matches.subcommand_matches("reinit") {
            reinit(None)?;
        } else if let Some(matches) = matches.subcommand_matches("edit") {
            manually(
                None,
                matches.value_of("key").unwrap(),
                matches.value_of("value").unwrap(),
            )?;
        }
    } else if let Some(_) = matches.subcommand_matches("setup") {
        setup(None, env!("CARGO_PKG_VERSION"))?;
    }

    Ok(())
}