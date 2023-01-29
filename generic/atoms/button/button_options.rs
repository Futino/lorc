use gloo::console::log;
use yew::prelude::*;
use yew_router::{prelude::Navigator, AnyRoute};

// This dependency relies on the local project to include the Enum: Route.
// Don't think it's a good practice but it's safe since all projects will have a route enum.
use crate::Route;

#[derive(PartialEq)]
pub enum RouteType {
    Route(Route),
    External(String),
}

#[derive(PartialEq)]
pub struct ButtonOptions {
    pub onclick: Callback<MouseEvent>,
    pub route: Option<RouteType>,
}

impl ButtonOptions {
    // Use example of route_button()
    /*
    let navigator: Navigator = _ctx.link().navigator().unwrap();
    let go_contact_page = ButtonOptions::route_button(navigator, RouteType::)
     */

    pub fn route_button(navigator: Navigator, route: RouteType) -> Self {
        match route {
            RouteType::Route(route) => ButtonOptions {
                onclick: Callback::from(move |_| {
                    navigator.push(&route);
                    log!("Navigator to Route: /Menu");
                }),
                route: None,
            },
            RouteType::External(route) => ButtonOptions {
                onclick: Callback::from(move |_| {
                    web_sys::window()
                        .expect("no global 'window' exists.")
                        .location()
                        .set_href(&route)
                        .expect("could not set URL using web_sys window.");
                    log!("Navigator to Route: /Menu");
                }),
                route: None,
            },
        }
    }
}

// ButtonOptions Presets

// Will route to a Route from the Route enum in main.rs or from a string.

/* Implementation if want to use string of internal path instead of route enum
   let path: &str = "/Menu";
   let empty_hashmap: HashMap<&str, &str> = HashMap::new();
   let go_maps = ButtonOptions {
       onclick: Callback::from(move |_| {
           navigator.push(&Route::from_path(path, &empty_hashmap).unwrap()) // instead of creating an empty hashmap, you might be able to just do (). haven't tested that though.
       }),
       href: None,
   };
*/
