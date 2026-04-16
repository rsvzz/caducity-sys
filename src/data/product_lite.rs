use crate::model::{CreateLiteExt, DataConext, QueryContext,};
use chrono::NaiveDateTime;
use rsqlite::{Connection, Error, params};
#[derive(Clone)]
pub struct ProductLite {
    pub barcode: String,
    pub name: String,
    pub description: String,
    pub categoryid: u32, // not (-) only (+) number
    pub date_created: Option<NaiveDateTime>,
    pub date_updated: Option<NaiveDateTime>,
}

impl ProductLite {
    pub fn new(_barcode: String, _name: String, _description: String, _id_category: u32) -> Self {
        ProductLite {
            barcode: _barcode,
            name: _name,
            description: _description,
            categoryid: _id_category,
            date_created: None::<NaiveDateTime>,
            date_updated: None::<NaiveDateTime>,
        }
    }
}

impl CreateLiteExt for ProductLite {
    type Output = ProductLite;
    type Key = String;

    fn del(key: Self::Key) -> Self::Output{
        ProductLite{
            barcode: key,
            name: "".to_string(),
            description: "".to_string(),
            categoryid: 0,
            date_created: None::<NaiveDateTime>,
            date_updated: None::<NaiveDateTime>,
        }
    }

    fn upd(key: Self::Key) -> Self::Output {
           ProductLite{
            barcode: key,
            name: "".to_string(),
            description: "".to_string(),
            categoryid: 0,
            date_created: None::<NaiveDateTime>,
            date_updated: None::<NaiveDateTime>,
        }
    }
    
    fn empty() -> Self {
        ProductLite {
            barcode: "".to_string(),
            name: "".to_string(),
            description: "".to_string(),
            categoryid: 0,
            date_created: None::<NaiveDateTime>,
            date_updated: None::<NaiveDateTime>,
        }
    }
}

impl DataConext for ProductLite {
    type Output = Connection;
    type Error = Error;

    fn connect(&self, db_path: &str) -> Result<Self::Output, Self::Error> {
        let conn = Connection::open(db_path);       
        conn
    }
    fn create(&self, db_path: &str) -> Result<String, String> {
        match self.connect(db_path) {
            Ok(conn) => {
                match conn.execute(
                    "CREATE TABLE IF NOT EXISTS Product (
                        barcode TEXT PRIMARY KEY,
                        name TEXT NOT NULL,
                        description TEXT NULL,
                        categoryid INTEGER NOT NULL,
                        date_created DATETIME DEFAULT (datetime('now')),
                        date_updated DATETIME DEFAULT NULL,
                        FOREIGN KEY (categoryid) REFERENCES Category(id));",
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
         INSERT INTO Product (barcode, name, description, categoryid)
         VALUES(?, ?, ?, ?);
         ",
                    params!(self.barcode, self.name, self.description, self.categoryid),
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
          DELETE FROM Product WHERE barcode = ?;
         ",
                    params!(self.barcode),
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
                let exec = match conn.execute("
         UPDATE Product SET name = ?, description = ?, categoryid = ?, date_updated= (datetime('now')) WHERE barcode = ?;
        ", params!(self.name, self.description, self.categoryid, self.barcode),)
         {
              Ok(rs) => Ok(format!("Insert into completed {}", rs.to_string())),
            Err(err) => Err(format!(
                "update fail {}",
                err.sqlite_error().unwrap().extended_code.to_string()
            ))
         };
                let _ =conn.close();
                exec
            }
            Err(err) => Err(format!(
                "connect fail {}",
                err.sqlite_error().unwrap().extended_code.to_string()
            )),
        }
    }
}

impl QueryContext for ProductLite {
    type Output = Option<Vec<ProductLite>>;

    fn get_all(&self, db_path: &str) -> Self::Output {
        if let Ok(conn) = Connection::open(db_path) {
            let stmt = conn
            .prepare("SELECT barcode, name, description, categoryid, date_created, date_updated FROM Product ORDER BY date_created DESC;");

            let list_iter = stmt
                .expect("REASON")
                .query_map([], |row| {
                    Ok(ProductLite {
                        barcode: row.get(0)?,
                        name: row.get(1)?,
                        description: row.get(2)?,
                        categoryid: row.get(3)?,
                        date_created: row.get::<_, Option<String>>(4)?.map(|s| {
                            NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").unwrap()
                        }),
                        date_updated: row.get::<_, Option<String>>(5)?.map(|s| {
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

/*
impl QueryFilterContext for ProductLite {
    type Output = Option<Vec<ProductLite>>;

    fn get_filter(&self, db_path: &str) -> Self::Output {
        if let Ok(conn) = Connection::open(db_path) {
            let stmt = conn
            .prepare("SELECT barcode, name, description, categoryid, date_created, date_updated FROM Product ORDER BY date_updated DESC;");

            let product_iter: Result<Vec<ProductLite>> = stmt
                .expect("REASON")
                .query_map([], |row| {
                    Ok(ProductLite {
                        barcode: row.get(0)?,
                        name: row.get(1)?,
                        description: row.get(2)?,
                        categoryid: row.get(3)?,
                        date_created: row.get::<_, Option<String>>(4)?.map(|s| {
                            NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").unwrap()
                        }),
                        date_updated: row.get::<_, Option<String>>(5)?.map(|s| {
                            NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").unwrap()
                        }),
                    })
                })
                .unwrap()
                .collect();

            if let Ok(list) = product_iter {
                Some(list)
            } else {
                None
            }
        } else {
            None::<Vec<ProductLite>>
        }
    }
}
*/
