pub mod add_school;
use strum::{EnumIter, IntoEnumIterator, IntoStaticStr};
use zoon::*;

#[static_ref]
fn school() -> &'static Mutable<Option<School>> {
    if let Some(Ok(school)) = local_storage().get("school") {
        return *Box::new(Mutable::new(Some(school)))
    }; 
    Mutable::new(None)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct School {
    pub id: i32,
    pub name: String,
}

pub fn school_page() -> impl Element {
    El::new().child_signal(school().signal_ref(|schl| match schl {
        Some(s) => {
           Row::new()
           .s(Gap::new().x(20))
           .s(Font::new().weight(FontWeight::Heavy))
           .s(Borders::new().bottom(Border::new().width(5)))
           .items(
            SchoolPages::iter().map(|page| {
                Button::new().label(format!("{page:?}"))
           }))
        }
        None => Row::new().item(add_school::add_school_page()),
    }))
    //add_school::add_school_page()
}


#[derive(Debug, Clone, Copy, EnumIter, IntoStaticStr)]
#[strum(crate="strum")]
enum SchoolPages {
    Classes,
    Teachers,
    Lectures,
    Timetabling,
}

