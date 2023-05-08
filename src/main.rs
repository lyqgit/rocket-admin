use rocket::{ get,routes,catchers };
use rocket::fs::FileServer;
use rocket::request::{Outcome, Request, FromRequest};

use log::{info, warn};
use log4rs;
use rbatis::Rbatis;

use once_cell::sync::OnceCell;
use rbdc_mysql::driver::MysqlDriver;

mod controller;
pub mod mapper;
pub mod model;
pub mod service;
pub mod entity;
pub mod util;

use controller::user_controller::{
    login,get_captcha,
    get_user_info,
    get_role_list,
    login_out
};
use controller::sys_menu_controller::{get_routers};
use controller::sys_role_controller::{get_role_page};
use controller::error_controller::{default_catcher,catcher_401};

#[cfg(test)]
mod tests {

    #[test]
    fn test_path(){
        assert_eq!(1,2)
    }
}

struct ABC(String);

pub static GLOBAL_DB:OnceCell<Rbatis> = OnceCell::new();

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ABC {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let uri_string = req.uri().to_string();
        println!("ABC守卫{}", uri_string);
        if uri_string == "/"{
            Outcome::Success(ABC("abc111".to_string()))//成功
        } else {
            Outcome::Forward(())//转发
            // Outcome::Failure((Default::default(), ())) // 失败
        }
    }
}

#[get("/a",rank=1)]
fn index() -> &'static str {
    "Hello, world!"
}


#[get("/",rank=0)]
fn index1(abc:ABC) {
    info!("测试");
    warn!("warn测试");
    println!("{}",abc.0)
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[macro_export]
macro_rules! pool {
    () => {
        &mut GLOBAL_DB.get().unwrap()
    };
}


#[rocket::main]
async fn main() {

    log4rs::init_file("src/config/log4rs.yaml", Default::default()).unwrap();

    info!("log4rs init");

    let rb = Rbatis::new();
    rb.link(MysqlDriver {}, "mysql://root:123456@localhost/ry-vue").await.unwrap();
    GLOBAL_DB.set(rb).expect("数据库链接失败");

    info!("rbatis init");

    rocket::build()
        .mount("/", routes![index,index1])
        .register("/",catchers![default_catcher,catcher_401])
        .mount("/", routes![hello])
        .mount("/static", FileServer::from("static/"))
        .mount("/api",routes![get_captcha,login,get_user_info,get_role_list,login_out,get_routers,get_role_page])
        .launch()
        .await
        .expect("服务启动失败");
}
