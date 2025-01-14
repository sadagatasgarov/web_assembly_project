use crate::elements::*;
use zoon::{println,*};
use crate::i18n::t;

const BLUE_5: &str = "#1E90FF"; // Replace with the actual HEX or RGB value
const RED_5: &str = "#FF4500"; 

#[static_ref]
fn first_name() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

#[static_ref]
fn last_name() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

#[static_ref]
fn email() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

fn change_first_name(name: String) {
    println!("{}", name);
    first_name().set(name);
}

fn change_last_name(lname: String) {
    println!("{}", lname);
    last_name().set(lname);
}

fn change_email(lname: String) {
    println!("{}", lname);
    email().set(lname);
}


pub fn signin_page() -> impl Element {
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
                .id("first_name")
                .input_type(InputType::text())
                .placeholder(Placeholder::with_signal(t!("first_name")))
                .on_change(change_first_name)
        )
        .item(
            TextInput::new()
                .s(Align::center())
                .s(Borders::all(Border::new().solid().color(BLUE_5)))
                .id("last_name")
                .input_type(InputType::text())
                .placeholder(Placeholder::with_signal(t!("last_name")))
                .on_change(change_last_name)
        )
        .item(
            TextInput::new()
                .s(Align::center())
                .s(Borders::all(Border::new().solid().color(BLUE_5)))
                .id("email")
                .input_type(InputType::text())
                .placeholder(Placeholder::with_signal(t!("email")))
                .on_change(change_email)
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
            TextInput::new()
                .s(Align::center())
                .s(Borders::all(Border::new().solid().color(BLUE_5)))
                .s(Height::exact(30))
                .id("password2")
                .input_type(InputType::password())
                .placeholder(Placeholder::with_signal(t!("password_again"))),
        )
        .item(
            Button::new()
            .s(RoundedCorners::all(20))
            .s(Borders::all(Border::new().solid().color(BLUE_5)))
            .s(Height::exact(30))
            .label(
                El::new()
                .s(Align::center())
                .child_signal(t!("login"))
            )
            .on_click(||{
                signin();
            })
        )
}

fn signin() {
    use super::User;
    use crate::app;
    use crate::router::router;
    use crate::router::Route;
    let user = User {
        id: 0,
        first_name: first_name().get_cloned(),
        last_name: last_name().get_cloned(),
        email: email().get_cloned()
    };
    app::login_user().set(Some(user.clone()));
    local_storage().insert("user", &user).expect("Session could not insert");
    router().replace(Route::Home);
}


