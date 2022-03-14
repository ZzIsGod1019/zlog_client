use std::io::{self, Read, Seek, SeekFrom};
use std::time::{SystemTime};
use std::path::{Path, PathBuf};
use std::fs::{self, File};
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
// 初始化
pub fn init() {
    // 创建偏移量记录文件
    File::create("./data/offset_index.toml").unwrap();
}

// 获取更新文件
pub fn get_updated_file(path: &str,interval: u64) -> Vec<PathBuf> {
    let mut res = Vec::new();
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let path = Path::new(path);
    let file_entry = fs::read_dir(path).unwrap().collect::<Result<Vec<_>, io::Error>>().unwrap();
    for entry in file_entry {
        let metadata = entry.metadata().unwrap();
        let modified_time = metadata.modified().unwrap().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        if metadata.is_file() && modified_time + interval > now {
            res.push(entry.path());
        }
    }
    res
}

// 更新偏移量记录
pub fn update_offset_file(file_name: String, offset: u64) {
    let mut file = File::open(path).unwrap();
    let config = config::get_config(Some("./data/offset_index"));
    println!("config:{:?}", config);
}

// 读取文件
pub fn read_file(path: PathBuf) -> [u8; 1024]{
    let mut buffer = [0; 1024];
    let path = path.as_path();
    let mut file = File::open(path).unwrap();
    file.seek(SeekFrom::Start(3)).unwrap();
    file.read_exact(&mut buffer).unwrap();

    buffer
}