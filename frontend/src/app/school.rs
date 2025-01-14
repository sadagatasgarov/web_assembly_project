pub mod add_school;
use strum::{EnumIter, IntoEnumIterator, IntoStaticStr};
use zoon::*;
use crate::i18n::t;

#[static_ref]
pub fn school() -> &'static Mutable<Option<School>> {
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
           .s(Font::new().weight(FontWeight::Medium))
           .s(Borders::new().bottom(Border::new().width(5)))
           .items(
            SchoolPages::iter().map(|page| {
                Button::new()
                .s(Width::exact(150))
                .label_signal(t!(page.label()))
           }))
        }
        None => Row::new().item(add_school::add_school_page()),
    }))
}


#[derive(Debug, Clone, Copy, EnumIter, IntoStaticStr)]
#[strum(crate="strum")]
enum SchoolPages {
    Home,
    Classes,
    Teachers,
    Lectures,
    Timetabling,
    
}


impl<'a> SchoolPages {
    fn label(&self) -> &'a str {
        match self  {
            SchoolPages::Classes => "classes",
            SchoolPages::Teachers => "teachers",
            SchoolPages::Lectures => "lectures",
            SchoolPages::Timetabling => "timetables",
            SchoolPages::Home => "home",
            
        }
    }
}


