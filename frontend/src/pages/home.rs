use yew::prelude::*;

use crate::components::enter_button::EnterButton;
use super::Root;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <h1>{ "National University School Clinic Management System" }</h1>
            <EnterButton<Root> label={ "Enter" } route={ Root::Dashboard } />
        </>
    }
}
