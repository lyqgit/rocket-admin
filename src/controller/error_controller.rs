use rocket::{catch,serde::json::Json,Request,http::Status};
use crate::model::common::{CustomResponse};

#[catch(default)]
pub fn default_catcher(status: Status, req: &Request<'_>)->Json<CustomResponse<()>>{
    println!("{:?}",status.to_string());
    println!("{:?}",req.uri());
    // req.guard::<Auth>().await.failed().unwrap();
    Json(CustomResponse{code: status.code as i32,data:None,msg:status.to_string()})
}

#[catch(401)]
pub fn catcher_401()->Json<CustomResponse<()>>{
    Json(CustomResponse{code: 401,data:None,msg:String::from("此用户没有权限访问")})
}