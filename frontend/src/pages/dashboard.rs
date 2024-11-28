use stylist::{css, Style};
use yew::prelude::*;

use super::Root;
use crate::components::prelude::*;

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

    html! {
        <main class={main_style}>
            <DashboardHeader />
            <section class={section_style}>
                <DashboardContainer title={"Students"} grid_column={(1, 4)}/>
                <DashboardContainer title={"Faculty"} grid_column={(4, 5)}/>
                <DashboardContainer title={"Appointments"} grid_column={(1, 3)}/>
                <DashboardContainer title={"Inventory"} grid_column={(3, 5)}/>
            </section>
        </main>
    }
}

#[function_component(DashboardHeader)]
pub fn dashboard_header() -> Html {
    let right_style = Style::new(css!(
        r#"
            margin-left: auto;
        "#
    ))
    .expect("Failed to create style.");

    html! {
        <Header>
            <Logo width={"50px"} height={"50px"}/>
            <Title font_size="1.2em">{"Clinic Management System"}</Title>
            <RedirectButton<Root> route={ Root::Home } class={right_style.clone()}> {"Back"} </RedirectButton<Root>>
        </Header>
    }
}

#[derive(Properties, PartialEq)]
pub struct DashboardContainerProps {
    pub title: String,
    pub grid_column: (u8, u8),
}

#[function_component(DashboardContainer)]
pub fn dashboard_container(props: &DashboardContainerProps) -> Html {
    let container_style = Style::new(css!(
        r#"
            border: 1px solid var(--color-blue);
            display: flex;
            flex-direction: column;
            gap: 0.25rem;
            min-height: 350px;
            max-height: 600px;
            grid-column: ${start} / ${end};
        "#,
        start = props.grid_column.0,
        end = props.grid_column.1,
    ))
    .expect("Failed to create style.");

    let right_style = Style::new(css!(
        r#"
            margin-left: auto;
        "#
    ))
    .expect("Failed to create style.");

    html! {
        <div class={container_style.clone()}>
            <Header>
                <Title font_size="1rem">{"Faculty"}</Title>
                <Button class={right_style.clone()}>
                    <MaterialIcon code={"add"}/>
                </Button>
            </Header>
        </div>
    }
}
