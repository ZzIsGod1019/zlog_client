use std::io;
use std::time::Duration;
use std::path::Path;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
// 判断文件是否更新
pub fn is_updated(path: String) {
    let path = Path::new(&path);
}