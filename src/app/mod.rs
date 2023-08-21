use yew::prelude::*;
use yew_router::prelude::*;

pub mod about;
pub mod home;
pub mod portfolio;
pub mod contact;

use crate::components::nav::Nav;
use about::About;
use home::Home;
use portfolio::Portfolio;
use contact::Contact;

/// App routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/portfolio")]
    Portfolio,
    #[at("/contact")]
    Contact,
    #[not_found]
    #[at("/404")]
    PageNotFound,
}

/// Switch app routes
pub fn switch(routes: AppRoute) -> Html {
    match routes.clone() {
        AppRoute::Home => html! { <Home /> },
        AppRoute::About => html! { <About /> },
        AppRoute::PageNotFound => html! { "Page not found" },
        AppRoute::Portfolio => html! { <Portfolio /> },
        AppRoute::Contact => html! { <Contact /> },
    }
}

/// Root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <HashRouter>
            <div class="flex min-h-screen flex-col">
                <Nav />
                <Home />
                <About />
                <Portfolio />
                <Contact />
            </div>
        </HashRouter>
    }
}
