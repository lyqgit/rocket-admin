use rbatis::{crud,impl_select};
use crate::entity::sys_captcha::SysCaptcha;

crud!(SysCaptcha{},"sys_captcha");
impl_select!(SysCaptcha{select_one(code:&String) -> Option => "`where code = #{code} order by create_time desc limit 1`"});
