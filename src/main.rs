use std::env;
use std::io::{self, Write};
use std::path::Path;
#[allow(unused_imports)]
use std::process::exit;
use std::process::Command;
use std::str;

mod build;
mod install;
mod project;

use build::build::{build, buildcustom, buildhard};
use build::run::run;
use install::install::install;
use project::create::create;
use project::header::header;
use project::reinit::reinit;
use project::testing::test;

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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn creation() -> std::io::Result<()> {
        create("test")?;
        let dir = &env::current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();
        assert!(Path::new(dir).exists());
        assert!(Path::new(&format!("{}\\project.json", dir)).exists());
        assert!(Path::new(&format!("{}\\deps.dat", dir)).exists());
        assert!(Path::new(&format!("{}\\src\\main.c", dir)).exists());

        Ok(())
    }
    #[test]
    fn building() -> std::io::Result<()> {
        env::set_current_dir("test")?;
        build();
        assert!(Path::new(".\\build\\debug\\debug.exe").exists());
        Ok(())
    }
    #[test]
    fn running() -> std::io::Result<()> {
        env::set_current_dir("test")?;
        run(vec![])?;
        Ok(())
    }
}

fn main() {
    let ver = Version {
        os: String::from("Windows"),
        main: 2,
        discriminator: 11,
        third: 2,
    };
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
    if argc < 2 {
        println!("Usage: wanager <command> [OPTIONS]");
        std::process::exit(1);
    }
    match argv[1].as_str() {
        "--version" => ver.display(),
        "new" => {
            if argc != 3 {
                return;
            }
            match create(&argv[2]) {
                Ok(()) => (),
                Err(_e) => println!("An error occured. Please retry later"),
            }
        }
        "build" => {
            if !Path::new("project.json").exists() || !Path::new("deps.dat").exists() {
                std::process::exit(-1);
            }
            if argc == 2 {
                build();
            } else if argc == 3 && argv[2].as_str() == "--release" {
                buildhard();
            } else if argc == 3 && argv[2].as_str() == "--custom" {
                buildcustom();
            }
        }
        "run" => {
            let mut args: Vec<&str> = Vec::new();
            for i in 2..argc {
                args.push(&argv[i]);
            }
            let ret = run(args);
            match ret {
                Ok(_) => (),
                Err(e) => println!("{}", e),
            }
        }
        "reinit" => {
            if !Path::new("project.json").exists() || !Path::new("deps.dat").exists() {
                std::process::exit(-1);
            }
            if argc == 3 && argv[2].as_str() == "--force" {
                match reinit() {
                    Ok(_) => (),
                    Err(_e) => println!("Error while reinitializing directory"),
                }
            } else {
                print!("Really want to reinit ? [y/N] : ");
                io::stdout().flush().unwrap();
                let mut answer = String::new();
                io::stdin()
                    .read_line(&mut answer)
                    .expect("Error while reading your choice. Please retry later");
                if answer.trim().to_uppercase().as_str() == "Y" {
                    match reinit() {
                        Ok(_) => (),
                        Err(e) => println!("Error while reinitializing directory : {}", e),
                    }
                } else {
                    println!("Reinitialisation aborted");
                }
            }
        }
        "header" => {
            if argc != 3 {
                return;
            }
            match header(&argv[2]) {
                Ok(_) => (),
                Err(e) => println!("{}", e),
            }
        }
        "install" => {
            if !Path::new("project.json").exists() || !Path::new("deps.dat").exists() {
                std::process::exit(-1);
            }
            if argc != 3 {
                return;
            }
            install(&argv[2]);
        }
        "test" => {
            if !Path::new("tests/tests.c").exists() {
                println!("Create file `tests/tests.c` before testing");
                std::process::exit(-2);
            }
            match test() {
                Ok(()) => (),
                Err(s) => println!("{}", s),
            }
        }
        _ => println!("Usage: wanager <command> [OPTIONS]"),
    }
}
