use stylist::{css, Style};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TitleProps {
    #[prop_or("2em".into())]
    pub font_size: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Title)]
pub fn title(props: &TitleProps) -> Html {
    let style = Style::new(css!(
        r#"
            font-size: ${font_size};
            text-wrap: wrap;
            color: var(--color-fg);
            font-weight: 800;
        "#,
        font_size = props.font_size,
    ))
    .expect("Failed to create style.");

    html! {
        <h1 class={style}>{ for props.children.iter() }</h1>
    }
}
