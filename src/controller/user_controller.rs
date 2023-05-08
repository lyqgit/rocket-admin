use rocket::{post,get};
use rocket::serde::json::Json;

use crate::model::user_model::{LoginUser, UserInfo, UserToken};
use crate::model::common::{CustomResponse};
use log::{info};
use crate::service::user_service::{get_user, get_user_by_id, login_out_by_id, set_user_token_and_expire};
use crate::entity::sys_captcha::SysCaptcha;
use crate::service::sys_captcha_service;
use crate::service::sys_role_service::get_role_by_name_page;
use md5::compute;
use crate::model::my_claims_model::MyClaims;
use jsonwebtoken::{encode,Header,EncodingKey};
use rbatis::rbdc::datetime::FastDateTime;
use rbatis::sql::Page;
use crate::entity::sys_role::SysRole;
use crate::util::captcha::create_captcha;
use crate::util::auth::Auth;
use crate::util::error::ResError;
use crate::util::res::{Res, ResOk, res_make_success, res_make, res_make_error};


#[post("/login",data="<loginUser>")]
pub async fn login(loginUser: Json<LoginUser>)->Res<UserToken>{
    // match &loginUser.username {
    //     Some(a)=>println!("{}",a),
    //     None=>println!("没有数值")
    // }
    // if loginUser.username.is_some(){
    //     info!("{}",loginUser.username.as_ref().unwrap());
    // }
    // info!("{:?}",loginUser.username.as_ref().unwrap());
    let time = FastDateTime::now();
    let captcha_obj = sys_captcha_service::get_last_one(&loginUser.get_code()).await.map_or_else(|_|None,|t|t);
    if captcha_obj.is_some(){
        let captcha_obj_s = captcha_obj.unwrap();

        if time.unix_timestamp() > captcha_obj_s.expire_time{
            return Err(res_make_error("验证码已过期，请重新获取".to_string()));
        }

    }else{
        return Err(res_make_error("验证码错误".to_string()));
    }

    println!("用户名-------------{:?}",loginUser.get_username());
    let username = loginUser.get_username();
    let a = get_user(username).await;
    println!("用户信息-------------{:?}",&a);
    let user = a.map_or_else(
        |_|None,
        |t|t
    );
    // a.map_or_else(
    //     |_|Json(CustomResponse{code:500,data:None,msg:"错误".to_string()}),
    //     |t|Json(CustomResponse{code:0,data:t,msg:"测试".to_string()})
    // )
    if user.is_some() {
        let password = loginUser.get_password();
        let md5_password = compute(&password);
        let mut user_s = user.unwrap();
        let pw = format!("{:x}", md5_password);
        println!("用户密码-------------{:?}",&pw);
        if user_s.password == pw {
            // 返回token

            let my_claims = MyClaims{id: user_s.user_id,username:(&user_s.user_name).to_string(),time:time.unix_timestamp()+3600*6,exp: (time.unix_timestamp() + 3600 * 6) as usize };
            let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("rocket_claims".as_ref()));
            let user_token = UserToken{token:token.map_or(String::from(""), |t|t)};
            user_s.token = Some((&user_token.token).to_string());
            user_s.expire_time = Some(time.unix_timestamp());
            let insert = set_user_token_and_expire(&user_s).await;
            match insert {
                Ok(a)=>println!("insert ok=>{:?}",a),
                Err(e)=>println!("insert Err=>{:?}",e)
            }
            Ok(res_make_success(Some(user_token),None))
        }else{
            Err(res_make_error("密码错误".to_string()))
        }

    }else {
        Err(res_make_error("无此用户".to_string()))
    }

    // match a {
    //     Ok(t)=>Json(CustomResponse{code:0,data:t,msg:"测试".to_string()}),//println!("{:?}",t),
    //     Err(e)=>Json(CustomResponse{code:500,data:None,msg:"错误".to_string()})//println!("{:?}",e)
    // }

}

#[get("/captchaImage")]
pub async fn get_captcha()->Res<String>{

    let (code,base64_img) = create_captcha();
    // let result =
    if base64_img.is_some(){
        sys_captcha_service::add(&SysCaptcha{id:0,code,create_time:FastDateTime::now(),expire_time:FastDateTime::now().unix_timestamp()+60}).await;
        Ok(res_make_success(base64_img,None))
    }else {
        Err(res_make_error(String::from("获取验证码失败")))
    }

    // match result {
    //     Ok(a)=>println!("insert ok=>{:?}",a),
    //     Err(e)=>println!("insert Err=>{:?}",e)
    // }
    // Json(CustomResponse{code:200,data:base64_img,msg:String::from("访问成功")})
}

#[get("/getInfo")]
pub async fn get_user_info(auth:Result<Auth,ResError>)->Res<UserInfo>{
    let a = auth?;
    let user_res = get_user_by_id(a.user_id).await;
    user_res.map_or_else(
        |e|Err(res_make_error(e.to_string())),
        |v|Ok(res_make_success(v,None))
    )
    // Ok(res_make_success(Some(a),None))
}

#[get("/role/list?<name>&<current>&<size>")]
pub async fn get_role_list(auth:Result<Auth,ResError>,name:&str,current:u64,size:u64)->Res<Page<SysRole>>{
    auth?;
    println!("name is {:?}",name);
    let role_list = get_role_by_name_page(name,current,size).await;
    role_list.map_or_else(
        |e|Ok(res_make_success(None,Some(e.to_string()))),
        |v|Ok(res_make_success(Some(v),None))
    )
}

#[post("/logout")]
pub async fn login_out(auth:Result<Auth,ResError>)->Res<()>{
    let auth_some = auth?;
    login_out_by_id(auth_some.user_id).await.map_or_else(
        |_|Ok(res_make(500,None,"退出登录失败".to_string())),
        |_|Ok(res_make_success(None,None))
    )
}
