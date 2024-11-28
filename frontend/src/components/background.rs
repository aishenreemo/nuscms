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
