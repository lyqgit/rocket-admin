use rocket::serde::{Deserialize,Serialize};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginUser{
    pub username:Option<String>,
    pub password:Option<String>,
    pub code:Option<String>
}

impl LoginUser{
    pub fn get_username(&self) ->String{
        self.username.as_ref().map_or(String::from(""),|s|s.to_string())
    }
    pub fn get_password(&self) ->String{
        self.password.as_ref().map_or(String::from(""),|s|s.to_string())
    }
    pub fn get_code(&self)->String{
        self.code.as_ref().map_or(String::from(""),|s|s.to_string())
    }
}

#[derive(Deserialize,Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserToken{
    pub token:String
}

#[derive(Deserialize,Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserInfo{
    pub real_name:Option<String>,
    pub avatar:Option<String>
}