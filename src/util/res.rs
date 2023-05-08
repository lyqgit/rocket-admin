use super::error::ResError;
use crate::model::common::CustomResponse;
use rocket::serde::{json::Json,Serialize};
use crate::util::auth::Auth;

pub type ResOk<T> = Json<CustomResponse<T>>;

pub type Res<T> = Result<ResOk<T>,ResError>;

pub type ResAuth = Result<Auth,ResError>;

pub fn res_make<T>(code:i32,data:Option<T>,msg:String)->ResOk<T>{
    Json(CustomResponse{code,data,msg})
}

pub fn res_make_success<T>(data:Option<T>,msg:Option<String>)->ResOk<T>{
    res_make(200,data,msg.map_or_else(||String::from("访问成功"),|s|s))
}

pub fn res_make_error(msg:String)->ResError{
    res_make(500,None,msg)
}
