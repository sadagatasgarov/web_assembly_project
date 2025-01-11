use zoon::*;
use crate::header;
use serde::{Serialize, Deserialize};


mod view;
pub static LANG_STORAGE_KEY: &str = "tr";


pub fn root() -> impl Element {
    Column::new()
        .s(Padding::new().top(15).right(10).left(10))
        .item(header::root())
        .item(view::root())
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

#[static_ref]
pub fn pages() -> &'static Mutable<Pages> {
    Mutable::new(Pages::Home)
}

fn logged_user() -> impl Signal<Item = Option<User>> {
    login_user().signal_cloned()
}


pub fn is_user_logged() -> bool {
    if let Some(_) = login_user().get_cloned() {
        return true
    }

    false
}


#[derive(Debug, Clone)]
pub enum Pages {
    Home,
    Login,
    Signin,
    NotFound
}

pub fn set_page_id(page: Pages) {
    pages().set(page);
}
