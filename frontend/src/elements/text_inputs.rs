use zoon::{Placeholder, TextInput, *};

const BLUE_5: &str = "#1E90FF"; // Replace with the actual HEX or RGB value
const RED_5: &str = "#FF4500";

pub fn default(id: &str, placeholder: &str) -> impl Element {
    TextInput::new()
        .s(Align::center())
        .s(Height::exact(35))
        .s(Borders::all(Border::new().solid().color(BLUE_5)))
        .id(id)
        .placeholder(Placeholder::new(placeholder))
        .input_type(InputType::text())
}
