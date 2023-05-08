use rocket::serde::{Deserialize,Serialize};

#[derive(Serialize,Clone)]
#[serde(crate = "rocket::serde")]
pub struct Router{
    pub path:String,
    pub name:Option<String>,
    pub permissions:Option<String>,
    pub roles:Option<String>,
    pub query:Option<String>,
    pub redirect:Option<String>,
    pub component:String,
    pub hidden:Option<bool>,
    pub alwaysShow:Option<bool>,
    pub children:Option<Vec<Router>>,
    pub meta:Meta,
}

#[derive(Serialize,Clone)]
#[serde(crate = "rocket::serde")]
pub struct Meta{
    pub title:Option<String>,
    pub icon:Option<String>,
    pub activeMenu:Option<String>,
    pub noCache:Option<bool>,
}