use rbatis::{html_sql,impled,Result};
use rbatis::executor::Executor;
use crate::entity::sys_menu::SysMenu;

#[html_sql("src/mapper/xml/sys_menu_xml.html")]
pub async fn select_all_by_user(rb: &mut dyn Executor, id: i32)->Result<Vec<SysMenu>>{
    impled!()
}
