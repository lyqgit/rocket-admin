use rocket::serde::{Deserialize};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Folder{
    pub name:Option<String>
}