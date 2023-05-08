use rbatis::{htmlsql_select_page};
use rbatis::executor::Executor;
use rbatis::sql::PageRequest;
use crate::entity::sys_role::SysRole;


htmlsql_select_page!(select_by_condition_page(name:&str)-> SysRole =>"src/mapper/xml/sys_role_xml.html");