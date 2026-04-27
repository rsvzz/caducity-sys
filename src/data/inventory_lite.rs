use crate::model::{CreateLiteExt, DataConext, QueryContext, QueryValidExtContext};
use chrono::NaiveDateTime;
use rsqlite::{Connection, Error, params};
///InventoryLite object view for sqlite
#[derive(Clone, Debug)]
pub struct InventoryLite {
    pub id: u32,
    pub statusid: u8,
    pub date_created: Option<NaiveDateTime>,
    pub date_updated: Option<NaiveDateTime>,
}

impl InventoryLite {
    ///new object default open
    pub fn new() -> Self {
        InventoryLite {
            id: 0,
            statusid: 1,
            date_created: None::<NaiveDateTime>,
            date_updated: None::<NaiveDateTime>,
        }
    }
}

impl CreateLiteExt for InventoryLite {
    type Output = InventoryLite;
    type Key = u32;

    fn del(key: Self::Key) -> Self::Output {
        InventoryLite {
            id: key,
            statusid: 0,
            date_created: None::<NaiveDateTime>,
            date_updated: None::<NaiveDateTime>,
        }
    }

    fn upd(key: Self::Key) -> Self::Output {
        InventoryLite{
            id: key,
            statusid: 0,
             date_created: None::<NaiveDateTime>,
            date_updated: None::<NaiveDateTime>,
        }
    }

    fn empty() -> Self {
        InventoryLite {
            id: 0,
            statusid: 0,
            date_created: None::<NaiveDateTime>,
            date_updated: None::<NaiveDateTime>,
        }
    }
}

impl DataConext for InventoryLite {
    type Output = Connection;
    type Error = Error;

    fn connect(&self, db_path: &str) -> Result<Self::Output, Self::Error> {
        Connection::open(db_path)
    }

    fn create(&self, db_path: &str) -> Result<String, String> {
        match self.connect(db_path) {
            Ok(conn) => {
                match conn.execute(
                    "CREATE TABLE IF NOT EXISTS Inventory ( 
                    id_inv INTEGER PRIMARY KEY AUTOINCREMENT, 
                    status_id INTEGER NOT NULL,
                    date_created DATETIME DEFAULT (datetime('now')),
                    date_updated DATETIME DEFAULT NULL);",
                    (),
                ) {
                    Ok(rs) => Ok(format!("table create {}", rs.to_string())),
                    Err(err) => Err(format!(
                        "table create fail {}",
                        err.sqlite_error().unwrap().extended_code.to_string()
                    )),
                }
            }
            Err(err) => Err(format!(
                "Connect fail {}",
                err.sqlite_error().unwrap().extended_code.to_string()
            )),
        }
    }

    fn insert(&self, db_path: &str) -> Result<String, String> {
        match self.connect(db_path) {
            Ok(conn) => {
                let exec = match conn.execute(
                    "
                    INSERT INTO Inventory (status_id)
                    VALUES(?);
                    ",
                    params!(self.statusid),
                ) {
                    Ok(rs) => Ok(format!("Insert into completed {}", rs.to_string())),
                    Err(err) => Err(format!(
                        "insert fail {}",
                        err.sqlite_error().unwrap().extended_code.to_string()
                    )),
                };
                let _ = conn.close();
                exec
            }
            Err(err) => Err(format!(
                "connect fail {}",
                err.sqlite_error().unwrap().extended_code.to_string()
            )),
        }
    }

    fn delete(&self, db_path: &str) -> Result<String, String> {
        match self.connect(db_path) {
            Ok(conn) => {
                let exec = match conn.execute(
                    "
          DELETE FROM Inventory AS inv WHERE inv.id_inv = ? AND NOT EXISTS(SELECT 1 FROM Inventory_detail WHERE id_inv = inv.id_inv LIMIT 1);
         ",
                    params!(self.id),
                ) {
                    Ok(rs) => Ok(format!("Insert into completed {}", rs.to_string())),
                    Err(err) => Err(format!(
                        "delete fail {}",
                        err.sqlite_error().unwrap().extended_code.to_string()
                    )),
                };
                let _ = conn.close();
                exec
            }
            Err(err) => Err(format!(
                "connect fail {}",
                err.sqlite_error().unwrap().extended_code.to_string()
            )),
        }
    }

    fn update(&self, db_path: &str) -> Result<String, String> {
        match self.connect(db_path) {
            Ok(conn) => {
                let exec = match conn.execute(
                    "
         UPDATE Inventory SET status_id = ?, date_updated= (datetime('now')) WHERE id_inv = ?;
        ",
                    params!(self.statusid, self.id),
                ) {
                    Ok(rs) => Ok(format!("Insert into completed {}", rs.to_string())),
                    Err(err) => Err(format!(
                        "update fail {}",
                        err.sqlite_error().unwrap().extended_code.to_string()
                    )),
                };
                let _ = conn.close();
                exec
            }
            Err(err) => Err(format!(
                "connect fail {}",
                err.sqlite_error().unwrap().extended_code.to_string()
            )),
        }
    }
}

impl QueryContext for InventoryLite {
    type Output = Option<Vec<InventoryLite>>;

    fn get_all(&self, db_path: &str) -> Self::Output {
        if let Ok(conn) = Connection::open(db_path) {
            let stmt = conn
            .prepare("SELECT id_inv, status_id, date_created, date_updated FROM Inventory ORDER BY id_inv DESC;");

            let list_iter = stmt
                .expect("REASON")
                .query_map([], |row| {
                    Ok(InventoryLite {
                        id: row.get(0)?,
                        statusid: row.get(1)?,
                        date_created: row.get::<_, Option<String>>(2)?.map(|s| {
                            NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").unwrap()
                        }),
                        date_updated: row.get::<_, Option<String>>(3)?.map(|s| {
                            NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").unwrap()
                        }),
                    })
                })
                .unwrap()
                .collect();

            if let Ok(list) = list_iter {
                Some(list)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl QueryValidExtContext for InventoryLite {
    type Output = Option<u8>;

    fn get_filter(&self, db_path: &str) -> Self::Output {
        if let Ok(conn) = Connection::open(db_path) {
            //closed = 0 and open = 1
            match conn.prepare("SELECT 1 FROM Inventory  WHERE status_id = 1 LIMIT 1;") {
                Ok(mut stmt) => {
                    match stmt.query_one([], |row| Ok(row.get::<_, u8>(0)?)) {
                        Ok(row) => Some(row),
                        Err(_) =>{
                              None
                        }
                    }
                }
                Err(_) => {
                    println!("Connection failt");
                    None
                },
            }
        } else {
            None
        }
    }
}
