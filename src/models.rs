use serde::Serialize;
use diesel::Queryable;
#[derive(Queryable, Serialize, Debug)]
pub struct Rustacean {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub created_at: String
}