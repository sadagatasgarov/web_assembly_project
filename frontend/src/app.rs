use zoon::*;
use crate::header;

pub static LANG_STORAGE_KEY: &str = "tr";


pub fn root() -> impl Element {
    Column::new()
        .s(Padding::new().top(15))
        .item(header::root())
        //.item(Label::new().label("Navbar"))
        .item("BODY")
}