use zoon::*;

mod app;
mod header;
mod i18n;
mod router;
mod elements;

fn main() {
    router::router();
    app::load_logged_user();
    start_app("app", app::root);
}
