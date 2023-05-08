use rbatis::rbdc::db::ExecResult;
use rbatis::executor::Executor;
use crate::entity::sys_user::SysUser;
use crate::GLOBAL_DB;
use rbatis::Result;
use crate::mapper::user_mapper::{get_user_info_by_id, update_by_id_login_out};
use crate::model::user_model::UserInfo;

pub async fn get_user(username:String)->Result<Option<SysUser>> {
    Ok(SysUser::select_by_id(&mut GLOBAL_DB.get().unwrap(),username).await?)
}

pub async fn get_user_by_token(token:String) ->Result<Option<SysUser>> {
    Ok(SysUser::select_by_token(&mut GLOBAL_DB.get().unwrap(),token).await?)
}

pub async fn set_user_token_and_expire(sysUser:&SysUser)->Result<ExecResult>{
    SysUser::update_by_id(&mut GLOBAL_DB.get().unwrap(),sysUser,sysUser.user_id).await
}

pub async fn login_out_by_id(id:i32)->Result<ExecResult>{
    update_by_id_login_out(&mut GLOBAL_DB.get().unwrap(),id).await
}


pub async fn get_user_by_id(id:i32)->Result<Option<UserInfo>> {
    get_user_info_by_id(&mut GLOBAL_DB.get().unwrap(),id).await
}