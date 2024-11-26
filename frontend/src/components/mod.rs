pub mod button;
pub mod prelude {
    pub use super::button::RedirectButton;
    pub use super::Background;
    pub use super::Header;
    pub use super::Logo;
    pub use super::Title;
}

use crate::pages::Root;
use button::RedirectButton;
use stylist::{css, Style};
use yew::prelude::*;

#[function_component(Background)]
pub fn background() -> Html {
    let style = Style::new(css!(
        r#"
            z-index: -1;
            filter: blur(5px);
            position: fixed;
            width: 100vw;
            height: 100vh;
            background-image: url("/static/images/background.png");
            background-size: cover;
            background-position: center;
            background-attachment: fixed;
            background-repeat: no-repeat;
            color: white;
        "#
    ))
    .expect("Failed to create style.");

    html! {
        <div class={style}></div>
    }
}

#[derive(Properties, PartialEq)]
pub struct LogoProps {
    #[prop_or("150px".into())]
    width: String,
    #[prop_or("150px".into())]
    height: String,
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

#[function_component(Header)]
pub fn header() -> Html {
    let style = Style::new(css!(
        r#"
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 60px;
            display: flex;
            align-items: center;
            gap: 1rem;
            background-color: var(--color-blue);
            color: white;
            padding: 0 1rem;
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
            z-index: 1000;
            justify-content: flex-start;
        "#
    )).expect("Failed to create style.");

    let button_style = Style::new(css!(
        r#"
            margin-left: auto;
        "#
    )).expect("Failed to create style.");

    html! {
        <header class={style}>
            <Logo width={"50px"} height={"50px"}/>
            <Title font_size="1.2em">{"Clinic Management System"}</Title>
            <RedirectButton<Root> route={ Root::Home } class={button_style}> {"Back"} </RedirectButton<Root>>
        </header>
    }
}
