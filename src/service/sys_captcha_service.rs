use crate::entity::sys_captcha::SysCaptcha;
use rbatis::Result;
use rbatis::rbdc::db::ExecResult;
use crate::GLOBAL_DB;
use crate::pool;

pub async fn add(sys_captcha:&SysCaptcha)->Result<ExecResult>{
    SysCaptcha::insert(pool!(), sys_captcha).await
}

pub async fn get_last_one(code:&String)->Result<Option<SysCaptcha>>{
    Ok(
        SysCaptcha::select_one(pool!(),code).await?
    )
}