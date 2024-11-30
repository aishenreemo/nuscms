pub mod dashboard;
pub mod home;

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
            <NotFound />
        },
    }
}

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <>
            <h1>{ "404 Not Found" }</h1>
        </>
    }
}
