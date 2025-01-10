use zoon::*;
use crate::i18n::t;

pub fn root() -> impl Element {
    Column::new()
        .s(Padding::new().right(10).left(10))
        .item(left_nav())
        .item(right_nav())
}


fn left_nav() -> impl Element {
    Row::new()
    .s(Align::new().left())
    .item(Link::new().label("Əsas səhifə").to("/"))
}

fn right_nav() -> impl Element {
    Row::new()
        .s(Align::new().right())
        .s(Gap::new().x(10))
        .item(Link::new().label("Sign in").to("/signin"))
        .item(Link::new().label_signal(t!("login")).to("/login"))
}