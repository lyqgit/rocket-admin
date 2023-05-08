use rocket::serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MyClaims{
    pub id:i32,
    pub username:String,
    pub time:i64,
    pub exp:usize
}