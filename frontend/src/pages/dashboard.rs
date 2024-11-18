use yew::prelude::*;

use crate::components::enter_button::EnterButton;
use super::Root;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    html! {
        <>
            <h1>{ "Dashboard" }</h1>
            <EnterButton<Root> label={ "Home" } route={ Root::Home }/>
        </>
    }
}
