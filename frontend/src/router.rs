use crate::*;
use app::Pages;
use std::collections::VecDeque;
use zoon::{println, *};

// ------ route_history ------

#[static_ref]
fn route_history() -> &'static Mutable<VecDeque<Route>> {
    Mutable::new(VecDeque::new())
}

fn push_to_route_history(route: Route) {
    let mut history = route_history().lock_mut();
    if history.len() == 2 {
        history.pop_back();
    }
    history.push_front(route);
}

pub fn previous_route() -> Option<Route> {
    route_history().lock_ref().get(1).cloned()
}

// ------ router ------

#[static_ref]
pub fn router() -> &'static Router<Route> {
    Router::new(|route: Option<Route>| async move {
        println!("{}", routing::url());

        //app::close_menu();

        let route = match route {
            Some(route) => {
                push_to_route_history(route.clone());
                route
            }
            None => Route::Home,
        };

        match route {
            Route::Login => {
                if app::is_user_logged() {
                    return router().replace(Route::Home);
                }
                app::set_page_id(Pages::Login);
            }

            Route::Signin => {
                if app::is_user_logged() {
                    return router().replace(Route::Home);
                }
                app::set_page_id(Pages::Signin);
            }

            // Route::ClientsAndProjects => {
            //     if not(app::is_user_logged()) {
            //         return router().replace(Route::Login);
            //     }
            //     clients_and_projects_page::request_clients();
            //     app::set_page_id(PageId::ClientsAndProjects);
            // }
            Route::Home => {
                //return router().replace(Route::Home);
                app::set_page_id(Pages::Home);
            }
        }
    })
}

// ------ Route ------

#[route]
#[derive(Copy, Clone)]
pub enum Route {
    #[route("login")]
    Login,

    #[route("signin")]
    Signin,

    #[route("/")]
    Home,
}
