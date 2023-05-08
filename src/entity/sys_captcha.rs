use rocket::serde::{Serialize,Deserialize};
use rbatis::rbdc::datetime::FastDateTime;

#[derive(Debug,Clone,Deserialize,Serialize)]
pub struct SysCaptcha{
    pub id:i32,
    pub code:String,
    pub create_time:FastDateTime,
    pub expire_time:i64
}