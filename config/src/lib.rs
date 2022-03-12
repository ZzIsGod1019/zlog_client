use std::{fs::File, io::Read};
use serde::Deserialize;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let config = get_config("./App.toml");
        
        assert_eq!(String::from("config1"), config.package.name);
    }
}

#[derive(Deserialize,Debug)]
pub struct Config {
    pub file_config : FileConfig
}

#[derive(Deserialize,Debug)]
pub struct FileConfig {
    pub path: String,
    pub listen_interval: u64,
}


pub fn get_config(path: Option<&'static str>)-> Config{
    let path = path.unwrap_or("./App.toml");
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
    match toml::from_str(&config_content) {
        Ok(c) => c,
        Err(e) => {
            panic!("toml文件解析失败:{}", e);
        }
    }

}