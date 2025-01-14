use zoon::*;

mod app;
mod elements;
mod header;
mod i18n;
mod router;

fn main() {
    router::router();
    app::load_logged_user();
    start_app("app", app::root);
}
