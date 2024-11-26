use stylist::{css, Style};
use yew::prelude::*;

use super::Root;
use crate::components::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let main_style = Style::new(css!(
        r#"
            display: grid;
            place-items: center;
            width: 100vw;
            height: 100vh;
        "#
    ))
    .expect("Failed to create style.");

    let home_style = Style::new(css!(
        r#"
            display: flex;
            flex-direction: column;
            justify-content: center;
            background-color: var(--color-blue);
            align-items: center;
            padding: 3rem 1rem;
            gap: 1rem;
            border-radius: 1rem;
            text-align: center;
        "#
    ))
    .expect("Failed to create style.");

    html! {
        <>
            <Background />
            <main class={main_style}>
                <div class={home_style}>
                    <Logo />
                    <Title >{"Clinic Management"} <br/> {"System"}</Title>
                    <RedirectButton<Root> route={ Root::Dashboard } > {"Enter"} </RedirectButton<Root>>
                </div>
            </main>
        </>
    }
}
