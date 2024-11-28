use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MaterialIconProps {
    pub code: String,
}

#[function_component(MaterialIcon)]
pub fn material_icon(props: &MaterialIconProps) -> Html {
    html! {
        <span class="material-symbols-outlined">{props.code.clone()}</span>
    }
}
