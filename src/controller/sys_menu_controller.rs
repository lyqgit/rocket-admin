use crate::util::res::{Res, res_make_error, res_make_success, ResAuth};
use rocket::{get};
use crate::model::sys_menu_model::Router;
use crate::service::sys_menu_service::get_routers_list;

#[get("/getRouters")]
pub async fn get_routers(auth:ResAuth)->Res<Vec<Router>>{
    let auth_ok = auth?;
    let res = get_routers_list(auth_ok.user_id).await;
    res.map_or_else(
        |e|Err(res_make_error(e.to_string())),
        |v|Ok(res_make_success(Some(v),None))
    )
}