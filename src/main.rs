use std::env;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command as CmdCommand;

use clap::{arg, Command};

extern crate yaml_rust;
use yaml_rust::{Yaml, YamlLoader};

use ssh2_config::SshConfig;

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
                .arg(arg!(<OPR>).possible_values(["init", "validate", "switch"]))
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
                "init" => exec_git_init(),
                "validate" => exec_git_validate(),
                "switch" => exec_git_switch(),
                _ => {}
            }
        }
        _ => println!("error"),
    }
}

// https://blog.csdn.net/icbm/article/details/71213492
fn exec_git_switch() {
    // check current dir is exist .git
    let mut ssh_keygen = CmdCommand::new("git");
    ssh_keygen.arg("rev-parse").arg("--is-inside-work-tree");

    let output = ssh_keygen.output().expect("cmd exec error!");

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    assert!(output.status.success());

    let mut ssh_keygen = CmdCommand::new("git");
    ssh_keygen.arg("rev-parse").arg("--git-dir");

    let output = ssh_keygen.output().expect("cmd exec error!");

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    assert!(output.status.success());

    let mut ssh_keygen = CmdCommand::new("git");
    ssh_keygen.arg("config").arg("core.sshCommand").arg("ssh -i ~/.ssh/github_niaoshuai_ed25519");

    let output = ssh_keygen.output().expect("cmd exec error!");

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    assert!(output.status.success());

    let mut ssh_keygen = CmdCommand::new("git");
    ssh_keygen.arg("config").arg("user.name").arg("niaoshuai");

    let output = ssh_keygen.output().expect("cmd exec error!");

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    assert!(output.status.success());

    let mut ssh_keygen = CmdCommand::new("git");
    ssh_keygen.arg("config").arg("user.email").arg("niao.shuai123@163.com");

    let output = ssh_keygen.output().expect("cmd exec error!");

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    assert!(output.status.success());
}

fn exec_git_validate() {
    let mut ssh_keygen = CmdCommand::new("ssh");
    ssh_keygen.arg("-T").arg("gitlab_diaobao");

    let output = ssh_keygen.output().expect("cmd exec error!");

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    assert!(output.status.success());
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

// exec git init
fn exec_git_init() {
    // check key_path
    let key_path = path_parse(OsStr::new("~/.ssh/diaobao_rsa"));
    // check change
    let key_path_clone = key_path.clone();
    if !key_path.exists() {
        // 1. 生成 ssh-key
        let mut ssh_keygen = CmdCommand::new("ssh-keygen");
        ssh_keygen
            .arg("-t")
            .arg("rsa")
            .arg("-C")
            .arg("renshuaipeng@jiaoyu361.com")
            .arg("-b")
            .arg("4096")
            .arg("-f")
            .arg(key_path);

        let output = ssh_keygen.output().expect("cmd exec error!");

        println!("status: {}", output.status);
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        assert!(output.status.success());
    }
    // 2. print key_value
    let pub_key_path = format!("{}.pub", key_path_clone.to_str().unwrap());
    let content = fs::read_to_string(pub_key_path).expect("Something went wrong reading the file");
    println!("{} content is \n{}", "renshuaipeng@jiaoyu361.com", content);
    // 3. generate ssh-config file
    write_ssh_config();
}

fn write_ssh_config() {
    let config_path = path_parse(OsStr::new("~/.ssh/config"));
    let config_path_clone = config_path.clone();
    let mut reader =
        BufReader::new(File::open(config_path).expect("Could not open configuration file"));
    let config = SshConfig::default()
        .parse(&mut reader)
        .expect("Failed to parse configuration");
    if config.query("gitlab_diaobao").host_name.is_none() {
        // begin write
        let mut file = OpenOptions::new()
            .append(true)
            .open(config_path_clone)
            .unwrap();
        let new_content = ssh_config_content_template(
            "gitlab_diaobao".to_string(),
            "git.youlu.com".to_string(),
            "~/.ssh/diaobao_rsa".to_string(),
        );
        file.write(new_content.as_bytes()).unwrap();
    } else {
        println!("diaobao")
    }
}
// handler path ~
fn path_parse(old_path_str: &OsStr) -> PathBuf {
    let old_path = Path::new(old_path_str);
    let mut new_path = PathBuf::from(old_path);
    if old_path.starts_with("~") {
        let home = env::var_os("HOME").unwrap();
        let tmp = old_path_str.to_str().unwrap();
        new_path = Path::new(&home).join(&tmp[2..])
    }
    return new_path;
}
// ssh_config template
fn ssh_config_content_template(host: String, host_name: String, key_file: String) -> String {
    let new_content = format!(
        "\n# {}
Host {}
    HostName {}
    User git
    IdentityFile {}
    PreferredAuthentications publickey",
        host, host, host_name, key_file
    );
    new_content
}
