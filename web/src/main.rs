use dioxus::prelude::*;

use ui::{NavLink, Navbar};
use views::{Blog, Home};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(WebNavbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/tailwind.css");
// const BULMA_CSS: &str = "https://cdn.jsdelivr.net/npm/bulma@1.0.4/css/bulma.min.css";
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        // document::Link { rel: "stylesheet", href: BULMA_CSS }
        document::Stylesheet {
                    // Urls are relative to your Cargo.toml file
                    href:MAIN_CSS
                }

        Router::<Route> {}
    }
}

/// A web-specific Router around the shared `Navbar` component
/// which allows us to use the web-specific `Route` enum.
#[component]
fn WebNavbar() -> Element {
    rsx! {
        Navbar {
            NavLink{
                Link {
                    class: "text-white hover:text-gray-200",
                    to: Route::Home {},
                    "Home"
                }

            }
        }

        Outlet::<Route> {}
    }
}
