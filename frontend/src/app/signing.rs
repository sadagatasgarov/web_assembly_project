use crate::elements::*;
use zoon::*;
use crate::i18n::t;

const BLUE_5: &str = "#1E90FF"; // Replace with the actual HEX or RGB value
const RED_5: &str = "#FF4500"; 



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
                .placeholder(Placeholder::with_signal(t!("first_name"))),
        )
        .item(
            TextInput::new()
                .s(Align::center())
                .s(Borders::all(Border::new().solid().color(BLUE_5)))
                .id("last_name")
                .input_type(InputType::text())
                .placeholder(Placeholder::with_signal(t!("last_name"))),
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
        first_name: "Sadagat".to_string(),
        last_name: "Asgarov".to_string(),
        email: "sadagat.asgarov@gmail.com".to_string()
    };
    //app::login_user().set(Some(user.clone()));
    //local_storage().insert("user", &user).expect("Session could not insert");
    router().replace(Route::Home);
}


