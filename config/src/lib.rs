#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let config = get_config("./App.toml");
        
        assert_eq!(String::from("config1"), config.package.name);
    }
}
use std::{fs::File, io::Read};
use serde::Deserialize;

#[derive(Deserialize,Debug)]
pub struct Config {
    file_config : FileConfig
}

#[derive(Deserialize,Debug)]
pub struct FileConfig {
    path: String,
    listen_interval: String,
}


pub fn get_config(path: &'static str)-> Config{
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
    match toml::from_str(&config_content) {
        Ok(c) => c,
        Err(e) => {
            panic!("toml文件解析失败:{}", e);
        }
    }

}