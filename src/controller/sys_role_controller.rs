use rocket::{get};
use crate::util::res::{Res, res_make_error, res_make_success, ResAuth};
use crate::service::sys_role_service::get_role_by_name_page;
use crate::entity::sys_role::SysRole;
use rbatis::sql::Page;

#[get("/getRolePage")]
pub async fn get_role_page(auth:ResAuth)->Res<Page<SysRole>>{
  auth?;
  let res = get_role_by_name_page("普通角色",1,10).await;
  res.map_or_else(
    |e|Err(res_make_error(e.to_string())),
    |v|Ok(res_make_success(Some(v),None))
  )
}