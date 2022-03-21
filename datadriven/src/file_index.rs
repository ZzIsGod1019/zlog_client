use rusqlite::{named_params, Connection, Result};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


pub struct data {
    pub file_name: String,
    pub offset: u32,
}

pub fn init() -> Result<()> {
    let conn = Connection::open("./data/app.db").
        unwrap_or_else(|e| panic!("未找到数据文件:{}",e));
        println!("111");
    conn.execute("create TABLE file_index (
        id              INTEGER PRIMARY KEY,
        file_name       TEXT NOT NULL,
        offset          INTEGER DEFAULT 0)", [])?;

    Ok(())
}

pub fn set_data(file_name: &str, offset: &str) -> Result<()> {
    let conn = Connection::open("./data/file_index").
        unwrap_or_else(|e| panic!("未找到数据文件:{}",e));
    conn.prepare("select offset from file_index where file_name = ?")?;

    Ok(())
}

