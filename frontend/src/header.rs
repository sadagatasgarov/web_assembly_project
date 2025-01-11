use crate::{
    app,
    i18n::{self, lang, t},
    router::Route,
};
use zoon::*;

pub fn root() -> impl Element {
    Row::new().item(left_nav()).item(right_nav())
}

fn left_nav() -> impl Element {
    Row::new()
        .s(Align::new().left())
        .item(Link::new().label("Əsas_səhifə").to("/"))
}

fn right_nav() -> impl Element {
    Row::new()
        .s(Gap::new().x(20))
        .s(Align::new().right())
        .item(lang_label())
        .item_signal(app::login_user().signal_ref(|user| {
            match user {
                //Some(u)=> Link::new().label(*u.first_name),
                Some(u) => Row::new().item(Link::new().label("A").to("/user/1")),
                None => Row::new()
                    .s(Gap::new().x(10))
                    .s(Align::new().right())
                    .item(Link::new().label_signal(t!("signin")).to(Route::Signin))
                    .item(Link::new().label_signal(t!("login")).to(Route::Login)),
            }
        }))
}

fn lang_label() -> impl Element {
    Button::new()
        .label_signal(lang().signal_ref(|l| Label::new().label(l.label())))
        .on_press(i18n::change_locale)
}
