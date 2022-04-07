use std::ffi::OsStr;
use std::fs;

use clap::{arg, Command};

extern crate yaml_rust;
use yaml_rust::{Yaml, YamlLoader};

use crate::kit::path_parse;

mod cmd;
mod kit;


fn cli() -> Command<'static> {
    Command::new("niao")
        .about("a tool")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .arg(
            arg!(
                -c --config <FILE> "config yaml"
            )
            .required(false)
            .default_value("~/.niao/config.yaml")
            .allow_invalid_utf8(true),
        )
        .subcommand(
            Command::new("git")
                .about("git about")
                .arg(arg!(<OPR>).possible_values(["init", "validate", "switch", "clone","restore","backup"]))
                .arg_required_else_help(true),
        )
}

fn main() {
    let matches = cli().get_matches();
    // Read Config
    let raw_config = matches.value_of_os("config").unwrap();
    read_config(raw_config);

    // handle subCommand
    match matches.subcommand() {
        Some(("git", sub_matches)) => {
            let opr = sub_matches.value_of("OPR").unwrap();
            match opr {
                "init" => cmd::exec_git_init(),
                "validate" => cmd::exec_git_validate(),
                "switch" => cmd::exec_git_switch(),
                "clone" => cmd::exec_git_clone(),
                "restore" => cmd::exec_git_restore(),
                "backup" => cmd::exec_git_backup(),
                _ => {}
            }
        }
        _ => println!("error"),
    }
}

// read config file
fn read_config(raw_config: &OsStr) -> Vec<Yaml> {
    let yaml_path = path_parse(raw_config);
    println!("read config from: {:?}", yaml_path);
    // read config content
    let contents = fs::read_to_string(yaml_path).expect("Something went wrong reading the file");
    // parse file
    let docs = YamlLoader::load_from_str(&contents).unwrap();
    docs
}


