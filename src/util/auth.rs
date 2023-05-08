use rocket::request::{FromRequest,Request,Outcome};
use jsonwebtoken::{decode,Header,DecodingKey,Validation};
use crate::model::my_claims_model::MyClaims;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::{json::Json,Serialize,Deserialize};
use crate::model::common::CustomResponse;
use log::{warn};
use crate::service::user_service::get_user_by_token;
use crate::util::error::ResError;
use crate::util::res::res_make;

#[derive(Debug,Serialize,Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Auth{
    pub token:String,
    pub user_id:i32,
    pub username:String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = ResError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let authorization = req.headers().get_one("Authorization");
        warn!("authorization is {:?}",authorization);
        if authorization.is_some(){
            let res_user = get_user_by_token(authorization.unwrap().to_string()).await;
            if res_user.is_ok(){
                if res_user.unwrap().is_some(){
                    let my_claims = decode::<MyClaims>(authorization.unwrap(), &DecodingKey::from_secret("rocket_claims".as_ref()), &Validation::default());
                    warn!("my_claims is {:?}",my_claims);
                    if my_claims.is_ok(){
                        let claims = my_claims.unwrap().claims;
                        let auth = Auth{token:authorization.unwrap().to_string(),user_id:claims.id,username:claims.username};
                        Outcome::Success(auth)
                    }else{
                        Outcome::Failure((Status::Unauthorized, res_make(401,None,String::from("此用户没有权限访问"))))
                    }
                }else {
                    Outcome::Failure((Status::Unauthorized,res_make(401,None,String::from("此用户没有权限访问"))))
                }

            }else{
                Outcome::Failure((Status::Unauthorized,res_make(500,None,String::from("系统错误"))))
            }


        }else {
            Outcome::Failure((Status::Unauthorized, res_make(401,None,String::from("此用户没有权限访问"))))
        }
        //
        // let uri_string = req.uri().to_string();
        // println!("ABC守卫{}", uri_string);
        // if uri_string == "/"{
        //     Outcome::Success(ABC("abc111".to_string()))//成功
        // } else {
        //     Outcome::Forward(())//转发
        //     // Outcome::Failure((Default::default(), ())) // 失败
        // }
    }
}