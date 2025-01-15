use crate::i18n::t;
use zoon::{println, *};


const BLUE_5: &str = "#1E90FF"; // Replace with the actual HEX or RGB value
const RED_5: &str = "#FF4500";

pub fn home() -> impl Element {
    Column::new().item("Timetables")
}