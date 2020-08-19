use std::env;
use std::io::{self, Write};
use std::path::Path;
#[allow(unused_imports)]
use std::process::exit;

mod build;
mod create;
mod header;
mod install;
mod reinit;
mod run;
mod  query;

use build::{build, buildhard};
use create::create;
use header::header;
use install::install;
use reinit::reinit;
use run::run;
use query::query;

struct Version {
    os: String,
    main: u8,
    discriminator: u8,
    third: u8,
}
impl Version {
    fn display(&self) {
        println!("Wanager by Wafelack <contactme.wafelack@protonmail.ch>, Licensed under GPL-v3.0, Version {} - {}.{}.{}", self.os, self.main, self.discriminator, self.third);
    }
}

fn main() {
    let ver = Version {
        os: String::from("Windows"),
        main: 2,
        discriminator: 9,
        third: 2,
    };
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
    if argc < 2 {
        println!("Usage: wanager <command> [OPTIONS]");
        std::process::exit(1);
    } else if argv[1] == "--version" {
        ver.display();
    } else if argv[1] == "new" && argc == 3 {
        let ret = create(&argv[2]);
        match ret {
            Ok(()) => (),
            Err(_e) => println!("An error occured. Please retry later"),
        }
    } else if argv[1] == "build" {
        if !Path::new("lock.wmg").exists() {
            std::process::exit(-1);
        }
        if argc == 3 && argv[2] == "--release" {
            build();
        } else {
            buildhard();
        }
    } else if argv[1] == "run" {
        let mut args: Vec<&str> = Vec::new();
        for i in 2..argc {
            args.push(&argv[i]);
        }
        let ret = run(args);
        match ret {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }
    } else if argv[1] == "reinit" {
        if !Path::new("lock.wmg").exists() {
            std::process::exit(-1);
        }
        if argc == 3 && argv[2] == "--force" {
            match reinit() {
                Ok(_) => (),
                Err(_e) => println!("Error while reinitializing directory"),
            }
            println!("Project reinitialized !");
        } else {
            print!("Really want to reinit ? [y/N] : ");
            io::stdout().flush().unwrap();
            let mut answer = String::new();
            io::stdin()
                .read_line(&mut answer)
                .expect("Error while reading your choice. Please retry later");
            if answer.trim().to_uppercase() == "Y" {
                match reinit() {
                    Ok(_) => (),
                    Err(e) => println!("Error while reinitializing directory : {}", e),
                }
            } else {
                println!("Reinitialisation aborted");
            }
        }
    } else if argv[1] == "header" && argc == 3 {
        match header(&argv[2]) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }
    } else if argv[1] == "install" && argc == 3 {
        match install(&argv[2]) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }
    } else if argv[1] == "query" && argc == 3 {
        query(&argv[2]);
    }else {
        println!("Usage: wanager <command> [OPTIONS]");
        std::process::exit(1);
    }
    std::process::exit(0);
}