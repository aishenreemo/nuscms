use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EnterButtonProps<T: Routable + Clone> {
    pub label: String,
    pub route: T,
}

#[function_component(EnterButton)]
pub fn enter_button<T: Routable + Clone + 'static>(props: &EnterButtonProps<T>) -> Html {
    let navigator = use_navigator().unwrap();
    let route = props.route.clone();
    let onclick = Callback::from(move |_| navigator.push(&route));

    html! {
        <button {onclick}>{ &props.label }</button>
    }
}
