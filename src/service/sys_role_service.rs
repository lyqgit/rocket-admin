use crate::entity::sys_role::SysRole;
use rbatis::Result;
use rbatis::rbdc::db::ExecResult;
use rbatis::sql::{Page, PageRequest};
use crate::GLOBAL_DB;
use crate::mapper::sys_role_mapper::select_by_condition_page;
use crate::pool;

pub async fn get_role_by_name_page(name:&str,current:u64,size:u64)->Result<Page<SysRole>>{
    select_by_condition_page(pool!(),&PageRequest::new(current,size),name).await
}