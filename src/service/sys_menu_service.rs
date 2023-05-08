use crate::mapper::sys_menu_mapper::select_all_by_user;
use crate::GLOBAL_DB;
use crate::model::sys_menu_model::{Meta,Router};
use crate::entity::sys_menu::SysMenu;
use rbatis::Result;



pub async fn get_routers_list(id:i32)->Result<Vec<Router>>{
    let origin_list = select_all_by_user(&mut GLOBAL_DB.get().unwrap(),id).await?;
    let mut new_list:Vec<Router> = Vec::new();
    println!("origin_list is{:?}",origin_list);
    trans_router(&mut new_list,&origin_list,0);
    Ok(new_list)
}

fn trans_router(list:&mut Vec<Router>,origin_list:&Vec<SysMenu>,pid:i32){

    for item in origin_list.iter(){

        let mut children_list:Vec<Router> = Vec::new();
        let meta = Meta{
            title: item.title.as_ref().cloned(),
            icon: item.icon.as_ref().cloned(),
            noCache: item.no_cache.map(|v|v==1),
            activeMenu: item.active_menu.as_ref().cloned()
        };

        if item.pid == pid{
            let mut is_has_child = 0;
            for item_a in origin_list.iter(){
                if item.id == item_a.pid{
                    is_has_child +=1
                }
            }

            println!("pid is {:?}",pid);
            println!("is_has_child is {:?}",is_has_child);
            println!("item.id is {:?}",item.id);


            if is_has_child > 0{
                trans_router(&mut children_list,origin_list,item.id);
            }


            list.push(Router{
                path: item.path.clone(),
                permissions: item.permissions.as_ref().cloned(),
                roles: item.roles.as_ref().cloned(),
                query: item.query.as_ref().cloned(),
                name: item.name.as_ref().cloned(),
                redirect: item.redirect.as_ref().cloned(),
                component: item.component.clone(),
                hidden: item.hidden.map(|v|v==1),
                alwaysShow: item.always_show.map(|v|v==1),
                children: Some(children_list),
                meta
            });
        }

    }
}