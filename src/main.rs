use std::time::{Duration, Instant};
use tokio::runtime::Runtime;
use std::thread;
use config;
use kernel;

pub fn main() {
    // 读取配置
    let config = config::get_config(Some("./config/App.toml"));
    // 初始化
    // kernel::init();
    // kernel::update_offset_file("./config/App.toml".to_string(), 0);
    // 启动读取日志文件
    let listen_interval = config.file_config.listen_interval;
    let path = config.file_config.path;
    let files = kernel::get_updated_file(&path, listen_interval);
    for file in files {
        println!("file:{:?}", file);
        let content = kernel::read_file(file);
        for li in content {
            println!("char:{:?}", li as char);
        }
    }
    
    

    let listen_thread = thread::spawn(move || {
        let rt  = Runtime::new().unwrap();
        rt.block_on(async {
            let task = rt.spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_secs(listen_interval));
                interval.tick().await;
                let start = Instant::now();
                println!("time:{:?}", start);
                loop {
                    interval.tick().await;
                    println!("hello,world");
                    println!("time:{:?}", start.elapsed());
                }
            });
            task.await.unwrap();
        });
    });
    
    listen_thread.join().unwrap();
}