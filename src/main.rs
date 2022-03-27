use clap::{arg, Command};
use std::env;
use std::fs;
use std::path::Path;

extern crate yaml_rust;
use yaml_rust::YamlLoader;

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
            .default_value(".niao/config.yaml")
            .allow_invalid_utf8(true),
        )
        .subcommand(
            Command::new("git")
                .about("git about")
                .arg(arg!(<OPR>).possible_values(["init", "validator", "switch"]))
                .arg_required_else_help(true),
        )
}

fn main() {
    let matches = cli().get_matches();
    // TODO 配置加载流程  先从当前目录拉取 -> 当前用户目录拉取
    // Read Config
    if let Some(raw_config) = matches.value_of_os("config") {
        let home = env::var_os("HOME").unwrap();
        let config_path = Path::new(raw_config);
        // let mut yaml_path= PathBuf::from(config_path);
        // if config_path.starts_with("~") {
        let yaml_path = Path::new(&home).join(config_path);
        println!("The config passed is: {:?}", yaml_path);
        // }

        // read config content
        let contents = fs::read_to_string(yaml_path.into_os_string())
            .expect("Something went wrong reading the file");
        // parse file
        let docs = YamlLoader::load_from_str(&contents).unwrap();
        println!("{:?}", docs);
    }
    // handle subCommand
    match matches.subcommand() {
        Some(("git", sub_matches)) => {
            println!("git was used, OPR is: {:?}", sub_matches.value_of("OPR"))
        }
        _ => println!("error"),
    }
}
