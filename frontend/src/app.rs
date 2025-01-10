use zoon::*;
use crate::header;
use serde::{Serialize, Deserialize};

pub static LANG_STORAGE_KEY: &str = "tr";


pub fn root() -> impl Element {
    Column::new()
        .s(Padding::new().top(15).right(10).left(10))
        .item(header::root())
        .item("BODY")
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String
}

// impl User {
//     pub fn name<'a>(&'a self)  -> &'a str {
//         format!("{} {}", self.first_name, self.last_name).as_str()
//     }
// }





#[static_ref]
pub fn login_user() -> &'static Mutable<Option<User>> {
    Mutable::new(None)
}


pub fn is_user_logged() -> bool {
    if let Some(_) = login_user().get_cloned() {
        return true
    }

    false
}