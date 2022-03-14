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
    
    let mut config_file = File::open(path)
        .unwrap_or_else(|e| panic!("未找到配置文件{}:{}", path,e));
    let mut config_content = String::new();

    config_file.read_to_string(&mut config_content)
        .unwrap_or_else(|e| panic!("读取失败{}:{}", path,e));
    
    toml::from_str(&config_content)
        .unwrap_or_else(|e| panic!("toml文件解析失败{}:{}", path,e))
}