use std::time::{Duration, Instant};
use tokio::runtime::Runtime;
use std::thread;
use config;
pub fn main() {
    // 读取配置
    let config = config::get_config(Some("./config/App.toml"));
    println!("config:{:?}", config);
    // 启动读取日志文件
    let listen_interval = config.file_config.listen_interval;
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