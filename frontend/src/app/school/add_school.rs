use crate::elements::*;
use crate::i18n::t;
use zoon::{println, *};

use super::{school, School};

const BLUE_5: &str = "#1E90FF"; // Replace with the actual HEX or RGB value
const RED_5: &str = "#FF4500";

#[static_ref]
fn school_name() -> &'static Mutable<String> {
    Mutable::new("".to_string())
}

fn change_school_name(name: String) {
    println!("{}", name);
    school_name().set(name);
}

pub fn add_school_page() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Gap::new().y(15))
        .item(
            Label::new()
                .s(Align::center())
                .label_signal(t!("add_school"))
                .s(Font::new().weight(FontWeight::SemiBold)),
        )
        .item(
            TextInput::new()
                .s(Align::center())
                .s(Borders::all(Border::new().solid().color(BLUE_5)))
                .id("add_school")
                .input_type(InputType::text())
                .placeholder(Placeholder::with_signal(t!("school_name")))
                .on_change(change_school_name),
        )
        .item(
            Button::new()
                .s(RoundedCorners::all(20))
                .s(Borders::all(Border::new().solid().color(BLUE_5)))
                .s(Height::exact(30))
                .label(El::new().s(Align::center()).child_signal(t!("add_school")))
                .on_click(add_school),
        )
}

fn add_school() {
    use crate::router::Route;
    use crate::router::router;
    super::school().set(Some(School {
        id: 0,
        name: school_name().get_cloned(),
    }));

    local_storage()
        .insert("school", &super::school().get_cloned())
        .expect("school add error session");

        router().replace(Route::Home);
}


// pub fn load_school() {
//     if let Some(Ok(user)) = local_storage().get("school") {
//         school_name().set(Some(user));
//     }
// }