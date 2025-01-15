pub mod add_school;
pub mod classes;
pub mod homepage;
pub mod lectures;
pub mod teachers;
pub mod timetables;


use crate::i18n::t;
use strum::{EnumIter, IntoEnumIterator, IntoStaticStr};
use zoon::{println, *};

const BLUE_5: &str = "#1E90FF"; // Replace with the actual HEX or RGB value
const RED_5: &str = "#FF4500";

#[static_ref]
pub fn school() -> &'static Mutable<Option<School>> {
    if let Some(Ok(school)) = local_storage().get("school") {
        return *Box::new(Mutable::new(Some(school)));
    };
    Mutable::new(None)
}

#[static_ref]
fn selected_page() -> &'static Mutable<SchoolPages> {
    Mutable::new(SchoolPages::default())
}

fn change_page(p: SchoolPages) {
    println!("aa");
    selected_page().set(p);
}

// fn page_signal(p: SchoolPages) -> impl Signal<Item = bool> {
//     Mutable::new(p == selected_page().get_cloned()).signal()
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct School {
    pub id: i32,
    pub name: String,
}

pub fn school_pages() -> impl Element {
    El::new().child_signal(school().signal_ref(|schl| {
        match schl {
            Some(s) => {
                Column::new()
                .item(school_tabs())
                .item_signal(
                    selected_page().signal_ref(|page| {
                        match page {
                            SchoolPages::Home => homepage::home(),
                            SchoolPages::Classes => classes::home(),
                            SchoolPages::Teachers => teachers::home(),
                            SchoolPages::Lectures => lectures::home(),
                            SchoolPages::Timetabling => timetables::home(),
                        }
                    })  
                )
            },
            None => Column::new()
            .item(Row::new().item(add_school::add_school_page())),
        }
    }))
}

fn school_tabs() -> impl Element {
    Row::new()
                .s(Gap::new().x(20))
                .s(Font::new().weight(FontWeight::Medium))
                .s(Borders::new().bottom(Border::new().width(0).solid().color(RED_5)))
                .items(SchoolPages::iter().map(|page| {
                    Button::new()
                        .s(Borders::new())
                        .s(Borders::new().bottom_signal(
                            selected_page().signal_ref(move |p| {
                                if p == &page {
                                    Border::new().width(2).solid().color(BLUE_5)
                                } else {
                                    Border::new().width(0).solid().color(RED_5)
                                }
                            }), // page_signal(page).map_bool(
                                //     || Border::new().width(1).solid().color(RED_5),
                                //     || Border::new().width(1).solid().color(BLUE_5)
                                //     )
                        ))
                        .s(Width::exact(150))
                        .on_click(move || change_page(page))
                        .label_signal(t!(page.label()))
                }))
}

#[derive(Debug, Clone, Copy, EnumIter, IntoStaticStr, Default, PartialEq)]
#[strum(crate = "strum")]
enum SchoolPages {
    #[default]
    Home,
    Classes,
    Teachers,
    Lectures,
    Timetabling,
}

impl<'a> SchoolPages {
    fn label(&self) -> &'a str {
        match self {
            SchoolPages::Classes => "classes",
            SchoolPages::Teachers => "teachers",
            SchoolPages::Lectures => "lectures",
            SchoolPages::Timetabling => "timetables",
            SchoolPages::Home => "home",
        }
    }
}
