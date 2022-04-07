use std::ffi::OsStr;
use std::fs;
use std::io::{self, Write};
use std::process::Command as CmdCommand;

use crate::kit::{path_parse, write_ssh_config};

pub // exec git init
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

// https://blog.csdn.net/icbm/article/details/71213492
pub fn exec_git_switch() {
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

pub fn exec_git_validate() {
    let mut ssh_keygen = CmdCommand::new("ssh");
    ssh_keygen.arg("-T").arg("gitlab_diaobao");

    let output = ssh_keygen.output().expect("cmd exec error!");

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    assert!(output.status.success());
}

pub fn exec_git_clone(){
    
}

pub fn exec_git_restore(){
    
}

pub fn exec_git_backup(){
    
}