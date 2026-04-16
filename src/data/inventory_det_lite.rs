use crate::model::{CreateLiteExt, DataConext, QueryContext, QueryFilterContext};
use chrono::{NaiveDate, NaiveDateTime};
use rsqlite::{Connection, Error, params};
#[derive(Debug, Clone)]
pub struct InventoryDetLite {
    pub id_ivt_det: u32,
    pub inv_id: u32,
    pub barcode: String,
    pub box_id: u8,
    pub stock: u32,
    pub status_id: u8,
    pub date_expiration: Option<NaiveDate>,
    pub date_created: Option<NaiveDateTime>,
    pub date_updated: Option<NaiveDateTime>,
}

impl InventoryDetLite {
    pub fn new(id_inv: u32, barcode: String, stock: u32, id_status: u8, id_box: u8, date_caducity: Option<NaiveDate>) -> Self {
        InventoryDetLite {
            id_ivt_det: 0,
            inv_id: id_inv,
            barcode: barcode,
            box_id: id_box,
            stock: stock,
            date_expiration: date_caducity,
            status_id: id_status,
            date_created: None::<NaiveDateTime>,
            date_updated: None::<NaiveDateTime>,
        }
    }
}

impl CreateLiteExt for InventoryDetLite {
    type Output = InventoryDetLite;

    type Key = u32;

    fn del(key: Self::Key) -> Self::Output {
        InventoryDetLite {
            id_ivt_det: key,
            inv_id: 0,
            barcode: String::new(),
            box_id: 0,
            stock: 0,
            date_expiration: None::<NaiveDate>,
            status_id: 0,
            date_created: None::<NaiveDateTime>,
            date_updated: None::<NaiveDateTime>,
        }
    }

    fn upd(key: Self::Key) -> Self::Output {
        InventoryDetLite {
            id_ivt_det: key,
            inv_id: 0,
            barcode: String::new(),
            box_id: 0,
            stock: 0,
            date_expiration: None::<NaiveDate>,
            status_id: 0,
            date_created: None::<NaiveDateTime>,
            date_updated: None::<NaiveDateTime>,
        }
    }

    fn empty() -> Self {
        InventoryDetLite {
            id_ivt_det: 0,
            inv_id: 0,
            barcode: String::new(),
            box_id: 0,
            stock: 0,
            date_expiration: None::<NaiveDate>,
            status_id: 0,
            date_created: None::<NaiveDateTime>,
            date_updated: None::<NaiveDateTime>,
        }
    }
}

impl QueryContext for InventoryDetLite {
    type Output = Option<Vec<InventoryDetLite>>;

    fn get_all(&self, db_path: &str) -> Self::Output {
        if let Ok(conn) = Connection::open(db_path) {
            let stmt = conn
            .prepare("
            SELECT id_ivt_det, id_inv, barcode, stock, date_expiration, status_id, box_id, date_created, date_updated 
            FROM Inventory_detail 
            ORDER BY id_inv DESC;");

            let list_iter = stmt
                .expect("REASON")
                .query_map([], |row| {
                    Ok(InventoryDetLite {
                        id_ivt_det: row.get(0)?,
                        inv_id: row.get(1)?,
                        barcode: row.get(2)?,
                        stock: row.get(3)?,
                        date_expiration: row
                            .get::<_, Option<String>>(4)?
                            .map(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").unwrap()),
                        status_id: row.get(5)?,
                        box_id: row.get(6)?,
                        date_created: row.get::<_, Option<String>>(7)?.map(|s| {
                            NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").unwrap()
                        }),
                        date_updated: row.get::<_, Option<String>>(8)?.map(|s| {
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

impl QueryFilterContext for InventoryDetLite {
    type Output = Option<Vec<InventoryDetLite>>;

    fn get_filter(&self, db_path: &str) -> Self::Output {
        if let Ok(conn) = Connection::open(db_path) {
            if let Ok(mut stmt) = conn
            .prepare("
            SELECT id_ivt_det, id_inv, barcode, stock, date_expiration, status_id, box_id, date_created, date_updated 
            FROM Inventory_detail WHERE id_inv = ?
            ORDER BY id_inv DESC;"){
                let list_iter= stmt
                .query_map([self.inv_id], |row| {
                    Ok(InventoryDetLite {
                        id_ivt_det: row.get::<_, u32>(0)?,
                        inv_id: row.get(1)?,
                        barcode: row.get(2)?,
                        stock: row.get(3)?,
                        date_expiration: row.get::<_, Option<String>>(4)?.map(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").unwrap()),
                        status_id: row.get(5)?,
                        box_id: row.get(6)?,
                        date_created: row.get::<_, Option<String>>(7)?.map(|s| {
                            NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").unwrap()
                        }),
                        date_updated: row.get::<_, Option<String>>(8)?.map(|s| {
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
            }
            else {
                None
            }
        } else {
            None
        }
    }
}

impl DataConext for InventoryDetLite {
    type Output = Connection;
    type Error = Error;

    fn connect(&self, db_path: &str) -> Result<Self::Output, Self::Error> {
        Connection::open(db_path)
    }

    fn create(&self, db_path: &str) -> Result<String, String> {
        match self.connect(db_path) {
            Ok(conn) => {
                match conn.execute(
                    "CREATE TABLE IF NOT EXISTS Inventory_detail ( 
                        id_ivt_det INTEGER PRIMARY KEY AUTOINCREMENT,
                        id_inv INTEGER, 
                        barcode TEXT NOT NULL,
                        box_id INTEGER NOT NULL,
                        stock INTEGER NOT NULL,
                        date_expiration DATE NOT NULL,
                        status_id INTEGER NOT NULL,
                        date_created DATETIME DEFAULT (datetime('now')),
                        date_updated DATETIME DEFAULT NULL,
                        FOREIGN KEY(id_inv) REFERENCES Inventory(id_inv),
                        FOREIGN KEY(barcode) REFERENCES Product(barcode));",
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
                    INSERT INTO Inventory_detail (id_inv, barcode, box_id, status_id, stock, date_expiration)
                    VALUES (?,?,?,?,?,?);
                    ",
                    params!(self.inv_id, self.barcode, self.box_id, self.status_id, self.stock, self.date_expiration.map(|f| f.to_string())),
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
                        DELETE FROM Inventory_detail WHERE id_ivt_det = ?;
                    ",
                    params!(self.id_ivt_det),
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
                    UPDATE Inventory_detail SET barcode = ?, box_id = ?, status_id = ? , stock = ?, date_expiration = ?, date_updated= (datetime('now')) WHERE id_ivt_det = ?;
        ",
                    params!(self.barcode, self.box_id, self.status_id, self.stock, self.date_expiration.map(|f| f.to_string()), self.id_ivt_det),
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
