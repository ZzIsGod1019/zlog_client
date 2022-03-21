use rusqlite::{named_params, Connection, Result};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn open_file(path: &str) -> Connection {
    Connection::open(path).
        unwrap_or_else(|e| panic!("未找到数据文件{}:{}", path,e))
}

pub fn create_table(conn: Connection, sql: &str) -> Result<usize> {
    conn.execute(sql, [])
}

// pub fn insert_data(conn: Connection, db: &str, key:&str, val:&str) {
//     let mut stmt = conn.prepare("SELECT id, name, data FROM :db where")?;
// }