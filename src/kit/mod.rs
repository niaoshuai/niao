use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use std::env;

use ssh2_config::SshConfig;

// handler path ~
pub fn path_parse(old_path_str: &OsStr) -> PathBuf {
    let old_path = Path::new(old_path_str);
    let mut new_path = PathBuf::from(old_path);
    if old_path.starts_with("~") {
        let home = env::var_os("HOME").unwrap();
        let tmp = old_path_str.to_str().unwrap();
        new_path = Path::new(&home).join(&tmp[2..])
    }
    return new_path;
}

pub fn write_ssh_config() {
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