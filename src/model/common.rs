use rocket::serde::{Serialize};

#[derive(Serialize,Debug)]
#[serde(crate = "rocket::serde")]
pub struct CustomResponse<T>{
    pub code:i32,
    pub data:Option<T>,
    pub msg:String
}

