use crate::app;
use crate::header;
use crate::i18n::t;
use serde::{Deserialize, Serialize};
use zoon::{eprintln, *};

use super::logged_user;

const BLUE_5: &str = "#1E90FF"; // Replace with the actual HEX or RGB value
const RED_5: &str = "#FF4500"; 

pub fn root() -> impl Element {
    Column::new()
        .s(Padding::new().top(15).right(10).left(10))
        .item_signal(app::pages().signal_cloned().map(|page| {
            match page {
                app::Pages::Home => home().into_raw(),
                app::Pages::Login => login().into_raw(),
                // app::Pages::Signin => todo!(),
                // app::Pages::NotFound => todo!(),
                _ => Label::new().label(format!("{:?}", page)).into_raw(),
            }
        }))
}

fn home() -> impl Element {
    Column::new()
        .s(Align::center())
        .item(Label::new().label("Libredu"))
        .item_signal(logged_user().map(|user| match user {
            Some(_) => Column::new().item(Label::new().label("Okul Ekle")),
            None => Column::new().item(Label::new().label_signal(t!("libredu-information"))),
        }))
}

fn login() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Gap::new().y(15))
        .item(
            Label::new()
                .s(Align::center())
                .label_signal(t!("login"))
                .s(Font::new().weight(FontWeight::SemiBold)),
        )
        .item(
            TextInput::new()
                .s(Align::center())
                .s(Borders::all(Border::new().solid().color(BLUE_5)))
                .id("email")
                .input_type(InputType::text())
                .placeholder(Placeholder::with_signal(t!("email"))),
        )
        .item(
            TextInput::new()
                .s(Align::center())
                .s(Borders::all(Border::new().solid().color(BLUE_5)))
                .s(Height::exact(30))
                .id("password")
                .input_type(InputType::password())
                .placeholder(Placeholder::with_signal(t!("password"))),
        )
        .item(
            Button::new().label_signal(t!("login")).on_click({||{}})
        )
}

