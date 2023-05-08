use rbatis::{crud,impl_select,impl_update,Result,html_sql};
use rbatis::executor::Executor;
use rbatis::rbdc::db::ExecResult;
use crate::entity::sys_user::SysUser;
use crate::model::user_model::UserInfo;

crud!(SysUser{},"sys_user");

impl_select!(SysUser{select_by_id(username:String) -> Option => "`where user_name = #{username} limit 1`"});
impl_select!(SysUser{select_by_token(token:String) -> Option => "`where token = #{token} limit 1`"});
impl_update!(SysUser{update_by_id(id:i32)=> "`where user_id = #{id}`"});

#[html_sql("src/mapper/xml/sys_user_xml.html")]
pub async fn update_by_id_login_out(rb: &mut dyn Executor, id: i32) -> Result<ExecResult> {
    impled!()
}

#[html_sql("src/mapper/xml/sys_user_xml.html")]
pub async fn get_user_info_by_id(rb: &mut dyn Executor, id: i32) -> Result<Option<UserInfo>> {
    impled!()
}
