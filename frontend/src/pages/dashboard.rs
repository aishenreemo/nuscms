use stylist::{css, Style};
use yew::prelude::*;

use crate::components::prelude::*;
use super::Root;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let main_style = Style::new(css!(
        r#"
            display: flex;
            flex-direction: column;
            width: 100%;
        "#
    ))
    .expect("Failed to create style.");

    let section_style = Style::new(css!(
        r#"
            margin: 1rem;
            display: grid;
            gap: 1rem;
            grid-template-columns: repeat(4, 1fr);
        "#
    ))
    .expect("Failed to create style.");

    let container_style = Style::new(css!(
        r#"
            border: 1px solid var(--color-blue);
            display: flex;
            flex-direction: column;
            gap: 0.25rem;
            min-height: 350px;
            max-height: 600px;
        "#
    ))
    .expect("Failed to create style.");

    let right_style = Style::new(css!(
        r#"
            margin-left: auto;
        "#
    )).expect("Failed to create style.");

    html! {
        <main class={main_style}>
            <Header>
                <Logo width={"50px"} height={"50px"}/>
                <Title font_size="1.2em">{"Clinic Management System"}</Title>
                <RedirectButton<Root> route={ Root::Home } class={right_style.clone()}> {"Back"} </RedirectButton<Root>>
            </Header>
            <section class={section_style}>
                <div class={container_style.clone()} style={"grid-column: 1 / 4;"}>
                    <Header>
                        <Title font_size="1rem">{"Students"}</Title>
                        <Button class={right_style.clone()}>
                            <MaterialIcon code={"add"}/>
                        </Button>
                    </Header>
                </div>
                <div class={container_style.clone()} style={"grid-column: 4 / 5;"}>
                    <Header>
                        <Title font_size="1rem">{"Faculty"}</Title>
                        <Button class={right_style.clone()}>
                            <MaterialIcon code={"add"}/>
                        </Button>
                    </Header>
                </div>
                <div class={container_style.clone()} style={"grid-column: 1 / 3;"}>
                    <Header>
                        <Title font_size="1rem">{"Appointments"}</Title>
                        <Button class={right_style.clone()}>
                            <MaterialIcon code={"add"}/>
                        </Button>
                    </Header>
                </div>
                <div class={container_style.clone()} style={"grid-column: 3 / 5;"}>
                    <Header>
                        <Title font_size="1rem">{"Inventory"}</Title>
                        <Button class={right_style.clone()}>
                            <MaterialIcon code={"add"}/>
                        </Button>
                    </Header>
                </div>
            </section>
        </main>
    }
}
