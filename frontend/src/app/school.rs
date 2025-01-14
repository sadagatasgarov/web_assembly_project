pub mod add_school;
use zoon::{*};

#[static_ref]
fn school() -> &'static Mutable<Option<School>> {
    Mutable::new(None)
}

pub fn school_page() -> impl Element {
    El::new().child_signal(
        school().signal_ref(|schl| {
            match schl {
                Some(s) => {
                    Column::new().item("a")
                },
                None => {
                   Column::new().item(add_school::add_school_page())
                },
            }
        })
    )
    //add_school::add_school_page()
}


pub struct School {
    pub id: i32,
    pub name: String
}

