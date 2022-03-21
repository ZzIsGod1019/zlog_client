use rusqlite::{named_params, Connection, Result};

const PATH:&str = "./data/app.db";

fn get_conn() -> Connection {
    Connection::open(PATH).
        unwrap_or_else(|e| panic!("未找到数据文件:{}",e))
}

pub fn init() -> Result<()> {
    let conn = get_conn();
    let mut stat = conn.prepare("select count(*) from sqlite_master  where type='table' and name = 'file_index'")?;
    let is_exists = stat.exists([])?;
    if !is_exists {
        conn.execute("create TABLE file_index (
            id              INTEGER PRIMARY KEY,
            file_name       TEXT NOT NULL,
            offset          INTEGER DEFAULT 0)", [])?;
    }
    Ok(())
}

pub fn get_offset(file_name: &str) -> Result<u32> {
    let conn = get_conn();
    let mut stat = conn.prepare("select offset from file_index where file_name = :file_name")?;

    let offset = stat.query_row(named_params!{
        ":file_name": file_name
    }, |row| row.get(0))?;

    Ok(offset)
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



