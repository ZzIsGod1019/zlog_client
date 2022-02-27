use std::{fs::File, io::Read};
use serde::Deserialize;

#[derive(Deserialize,Debug)]
struct Config {
    package : Package
}

#[derive(Deserialize,Debug)]
struct Package {
    name: String,
    version: String,
    edition: String,
}
fn main() {
    let path = "./config/App.toml";
    let mut config_file = match File::open(path) {
        Ok(c) => c,
        Err(e) => {
            panic!("未找到配置文件{}:{}", path,e);
        }
    };
    let mut config_content = String::new();
    match config_file.read_to_string(&mut config_content) {
        Ok(c) => c,
        Err(e) => {
            panic!("读取失败:{}", e);
        }
    };
    println!("文件内容:{}", config_content);
    let config:Config = toml::from_str(&config_content).unwrap();

    println!("config:{:?}", config);
}
