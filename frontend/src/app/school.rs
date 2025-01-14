pub mod add_school;
use zoon::{*};

pub fn school_page() -> impl Element {
    add_school::add_school_page()
}