///defult impl for obj sqlite
pub trait DataConext {
    type Output;
    type Error;
    ///Create Table if not exist
    fn connect(&self, db_path: &str) -> Result<Self::Output, Self::Error>;
    fn create(&self, db_path: &str) -> Result<String, String>;
    fn insert(&self, db_path: &str) -> Result<String, String>;
    fn delete(&self, db_path: &str) -> Result<String, String>;
    fn update(&self, db_path: &str) -> Result<String, String>;
}

///Ext empty object lite db
pub trait CreateLiteExt {
    type Output;
    type Key;

    /// fn delete row 
    fn del(key: Self::Key) -> Self::Output;
    /// create obj for update with only key
    fn upd(key: Self::Key) -> Self::Output;
    fn empty() -> Self;
}
