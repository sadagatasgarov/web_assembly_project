use zoon::*;
use crate::header;
use serde::{Serialize, Deserialize};
use crate::app;
use crate::i18n::t;

use super::logged_user;


pub fn root() -> impl Element {
    Column::new()
        .s(Padding::new().top(15).right(10).left(10))
        .item_signal(
            app::pages().signal_cloned().map(|page|{
                match page {
                    app::Pages::Home => {
                        home().into_raw()
                    },
                    // app::Pages::Login => todo!(),
                    // app::Pages::Signin => todo!(),
                    // app::Pages::NotFound => todo!(),
                    _ => Label::new().label(format!("{:?}", page)).into_raw()
                }
            })
        )
}


fn home() -> impl Element {
    Column::new()
    .s(Align::center())
    .item(
        Label::new().label("Libredu")
    )
    .item_signal(
        logged_user().map(|user |
        match user {
            Some(u) => Column::new().item(Label::new().label("Okul Ekle")),
            None => Column::new().item(Label::new().label_signal(t!("libredu-information"))),
        }
        )
        

    )
}