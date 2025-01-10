use zoon::*;
use crate::i18n::{self, lang, t, Lang};

pub fn root() -> impl Element {
    Column::new()
        .s(Padding::new().right(10).left(10))
        .item(left_nav())
        .item(right_nav())
}


fn left_nav() -> impl Element {
    Row::new()
    //.s(Align::new().left())
    .item(
        Link::new().label("Əsas_səhifə").to("/")
    )
}

fn right_nav() -> impl Element {
    Row::new()
    .s(Gap::new().x(20))
    .item(lang_label())
    .item(
        Row::new()
        .s(Gap::new().x(10))   
        .s(Align::new().right())
        .item(
            Link::new().label("Sign in").to("/signin")
        )
        .item(
            Link::new().label_signal(t!("login")).to("/login")
        )
    )
}




fn lang_label() -> impl Element {
    Button::new()
    .label_signal(
        lang()
        .signal_ref(|l| {
            Label::new().label(l.label())
    })).on_press(i18n::change_locale)

}