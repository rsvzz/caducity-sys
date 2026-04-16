pub trait QueryContext{
    type Output;
    fn get_all(&self, db_path: &str) -> Self::Output;
}

///impl filter for obj sqlite
pub trait QueryFilterContext{
    type Output;
    ///filter default
    fn get_filter(&self, db_path: &str) -> Self::Output;
}
///filter valid data return Obj
pub trait QueryValidExtContext{
    type Output;
    ///filter default
    fn get_filter(&self, db_path: &str) -> Self::Output;
}