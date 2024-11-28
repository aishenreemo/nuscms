use stylist::{css, Style};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let style = Style::new(css!(
        r#"
            width: 100%;
            height: 60px;
            display: flex;
            align-items: center;
            gap: 1rem;
            background-color: var(--color-blue);
            color: white;
            padding: 0 1rem;
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
            justify-content: flex-start;
        "#
    ))
    .expect("Failed to create style.");

    html! {
        <header class={style}>
            { for props.children.iter() }
        </header>
    }
}
