pub mod home;
pub mod dashboard;
pub mod not_found;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone)]
pub enum Root {
    #[at("/")]
    Home,
    #[at("/dashboard")]
    Dashboard,
    #[at("/404")]
    #[not_found]
    NotFound,
}

pub fn switch(routes: Root) -> Html {
    match routes {
        Root::Home => html! { 
            <home::Home />
        },
        Root::Dashboard => html! {
            <dashboard::Dashboard />
        },
        Root::NotFound => html! {
            <not_found::NotFound />
        },
    }
}
