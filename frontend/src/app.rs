use crate::header;
use serde::{Deserialize, Serialize};
use zoon::{strum::{EnumIter, IntoEnumIterator, IntoStaticStr},*};

mod school;
mod signing;
mod view;

pub static LANG_STORAGE_KEY: &str = "tr";

pub fn root() -> impl Element {
    Column::new()
        .s(Padding::new().top(15).right(10).left(10))
        .item(header::root())
        .item(view::root())
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct User {
    id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl User {
    pub fn name<'a>(&'a self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

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
        return true;
    }

    false
}

#[derive(Debug, Clone)]
pub enum Pages {
    Home,
    Login,
    Signin,
    Logout,
    NotFound,
    AddSchool,
}

pub fn set_page_id(page: Pages) {
    pages().set(page);
}

pub fn load_logged_user() {
    if let Some(Ok(user)) = local_storage().get("user") {
        login_user().set(Some(user));
    }
}