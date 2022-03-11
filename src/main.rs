use std::time::{Duration, Instant};
use tokio::runtime::Runtime;
use config;
pub fn main() {
    // 读取配置
    config::get_config(path)
    let rt  = Runtime::new().unwrap();
    rt.block_on(async {
        let task = rt.spawn(async {
            let mut interval = tokio::time::interval(Duration::from_millis(1000));
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

}