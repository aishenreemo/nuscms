use stylist::{css, Style};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LogoProps {
    #[prop_or("150px".into())]
    pub width: String,
    #[prop_or("150px".into())]
    pub height: String,
}

#[function_component(Logo)]
pub fn logo(props: &LogoProps) -> Html {
    let style = Style::new(css!(
        r#"
            width: ${width};
            height: ${height};
        "#,
        width = props.width,
        height = props.height,
    ))
    .expect("Failed to create style.");

    html! {
        <img src={"/static/images/logo.png"} class={style}/>
    }
}
