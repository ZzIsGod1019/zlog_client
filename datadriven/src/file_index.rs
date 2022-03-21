use rusqlite::{named_params, Connection, Result};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

const PATH:&str = "./data/app.db";

pub struct data {
    pub file_name: String,
    pub offset: u32,
}

fn get_conn() -> Connection {
    Connection::open(PATH).
        unwrap_or_else(|e| panic!("未找到数据文件:{}",e))
}

pub fn init() -> Result<()> {
    let conn = get_conn();
    conn.execute("create TABLE file_index (
        id              INTEGER PRIMARY KEY,
        file_name       TEXT NOT NULL,
        offset          INTEGER DEFAULT 0)", [])?;
    Ok(())
}

pub fn get_offset(file_name: &str) -> Result<u32> {
    let conn = get_conn();
    let mut stat = conn.prepare("select offset from file_index where file_name = :file_name")?;
    let rows = stat.query_map(named_params!{
        ":file_name": file_name
    }, |row| row.get(0))?;

    let mut names = 0;
    for offset_result in rows {
        names = offset_result?;
    }
    Ok(names)
}

pub fn set_data(file_name: &str, offset: u32) -> Result<()> {
    let conn = get_conn();
    let mut stat = conn.prepare("select offset from file_index where file_name = :file_name")?;
    let is_exists = stat.exists(named_params!{
        ":file_name": file_name
    })?;
    if is_exists {// update
        conn.execute(
            "UPDATE file_index SET offset = ':offset' WHERE file_name = :file_name",
            named_params! {
                ":offset": offset,
                ":file_name": file_name
            }
        )?;
    }else {// insert
        conn.execute(
            "INSERT INTO file_index (file_name, offset)
             VALUES (:file_name, :offset)",
            named_params! {
                ":file_name": file_name,
                ":offset": offset
            }
        )?;
    }
    Ok(())
}



