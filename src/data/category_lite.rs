use crate::model::{CreateLiteExt, DataConext, QueryContext, QueryFilterContext};
use chrono::NaiveDateTime;
use rsqlite::{Connection, Result, params, Error};
use std::result;
#[derive(Clone, Debug)]
pub struct CategoryLite {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub date_created: Option<NaiveDateTime>,
    pub date_updated: Option<NaiveDateTime>,
}

impl CategoryLite {
    /// Created object with data simple
    pub fn new(name: String, description: String) -> Self {
        CategoryLite {
            id: 0,
            name: name,
            description: description,
            date_created: None::<NaiveDateTime>,
            date_updated: None::<NaiveDateTime>,
        }
    }

    /// created object from db.
    pub fn data_new(
        _id: u32,
        name: String,
        description: String,
        _date_created: NaiveDateTime,
        _date_updated: NaiveDateTime,
    ) -> Self {
        CategoryLite {
            id: _id,
            name: name,
            description: description,
            date_created: Some(_date_created),
            date_updated: Some(_date_updated),
        }
    }

}

impl CreateLiteExt for CategoryLite {
    type Output = CategoryLite;
    type Key = u32;

    fn del(key: Self::Key) -> Self::Output {
        CategoryLite {
            id: key,
            name: "".to_string(),
            description: "".to_string(),
            date_created: None::<NaiveDateTime>,
            date_updated: None::<NaiveDateTime>,
        }
    }

    fn upd(key: Self::Key) -> Self::Output {
         CategoryLite {
            id: key,
            name: "".to_string(),
            description: "".to_string(),
            date_created: None::<NaiveDateTime>,
            date_updated: None::<NaiveDateTime>,
        }
    }
    
    fn empty() -> Self {
        CategoryLite {
            id: 0,
            name: "".to_string(),
            description: "".to_string(),
            date_created: None::<NaiveDateTime>,
            date_updated: None::<NaiveDateTime>,
        }
    }
}

impl DataConext for CategoryLite {
    type Output = Connection;
    type Error = Error;

    fn connect(&self, db: &str) -> Result<Self::Output, Self::Error> {
        Connection::open(db)
    }

    fn create(&self, db_path : &str) -> result::Result<String, String> {
        match self.connect(db_path){
            Ok(conn) => {
                 let exec = match conn.execute(
            "CREATE TABLE IF NOT EXISTS Category ( 
                        id INTEGER PRIMARY KEY AUTOINCREMENT, 
                        name TEXT NOT NULL, 
                        description TEXT NULL, 
                        date_created DATETIME DEFAULT (datetime('now')),
                        date_updated DATETIME DEFUALT NULL);",
            (),
        ) {
            Ok(rs) => Ok(format!("table create {}", rs.to_string())),
            Err(err) => Err(format!(
                "table create fail {}",
                err.sqlite_error().unwrap().extended_code.to_string()
            )),
        };
         let _ = conn.close();
        exec
            }
           Err(err) => Err(format!(
                "connect fail {}",
                err.sqlite_error().unwrap().extended_code.to_string()
            ))
            
        }
        
    }

    fn insert(&self, db_path : &str) -> result::Result<String, String> {
          match self.connect(db_path){
            Ok(conn) => {
        let exec = match conn.execute(
            "
         INSERT INTO Category (name, description) 
         VALUES(?, ?);
         ",
            params!(self.name, self.description),
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
            ))
    }
    }

    fn delete(&self, db_path : &str) -> result::Result<String, String> {
          match self.connect(db_path){
            Ok(conn) => {
        let exec = match conn.execute(
            "
         DELETE FROM Category WHERE id = ?;
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
            ))
    }
}

    fn update(&self, db_path : &str) -> result::Result<String, String> {
           match self.connect(db_path){
            Ok(conn) => {
        let exec = match conn.execute("
         UPDATE Category SET name = ?, description = ?, date_updated= (datetime('now')) WHERE id = ?;
         ", params!(self.name, self.description, self.id),)
         {
              Ok(rs) => Ok(format!("Update into completed {}", rs.to_string())),
            Err(err) => Err(format!(
                "update fail {}",
                err.sqlite_error().unwrap().extended_code.to_string()
            ))
         };
        let _ = conn.close();
        exec
    }
      Err(err) => Err(format!(
                "connect fail {}",
                err.sqlite_error().unwrap().extended_code.to_string()
            ))
    }
    }
}

impl QueryContext for CategoryLite {
    type Output = Option<Vec<CategoryLite>>;
    fn get_all(&self, db_path: &str) -> Self::Output {
          match self.connect(db_path)
          {
            Ok(conn) => {
        match conn
            .prepare("SELECT id, name, description, date_created, date_updated FROM Category ORDER BY id DESC;"){
         Ok(mut stmt) => {
                 let list_iter: Result<Vec<CategoryLite>> = stmt
            .query_map([], |row| {
                Ok(CategoryLite {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    date_created: row
                        .get::<_, Option<String>>(3)?
                        .map(|s| NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").unwrap()),
                    date_updated: row
                        .get::<_, Option<String>>(4)?
                        .map(|s| NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").unwrap()),
                })
            }).unwrap()
            .collect();
          match list_iter 
          {
            Ok(list) => Some(list),
            Err(_) => None
          }
        }
           Err(_) => None
        }
    }
      Err(_) => None
}
}
}


impl QueryFilterContext for CategoryLite {
    type Output = Option<Vec<CategoryLite>>;
    /// ORDER name for filter customer
    fn get_filter(&self, db_path: &str) -> Self::Output {
           match self.connect(db_path){
            Ok(conn) => {
                  if let Ok(mut stmt) = conn.prepare("SELECT id, name FROM Category ORDER BY name DESC;") {
            let category_iter: Result<Vec<CategoryLite>> = stmt
                .query_map([], |row| {
                    Ok(CategoryLite {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        description: "".to_string(),
                        date_created: None,
                        date_updated: None,
                    })
                })
                .unwrap()
                .collect();

          match category_iter 
          {
            Ok(list) => Some(list),
            Err(_) => None
          }

        }
        else {
            None
        }
    }
    Err(_) => None
    }
}
}