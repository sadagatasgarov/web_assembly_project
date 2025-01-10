use zoon::*;

mod app;
mod header;
mod i18n;
mod router;


fn main() {
    router::router();
    start_app("app", app::root);
}


