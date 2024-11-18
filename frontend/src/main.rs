mod components;
mod pages;

use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<pages::Root> render={pages::switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
