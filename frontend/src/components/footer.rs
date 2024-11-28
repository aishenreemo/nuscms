use stylist::{css, Style};
use yew::prelude::*;
use super::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    let style = Style::new(css!(
        r#"
            background-color: var(--color-blue);
            color: var(--color-fg);
            display: flex;
            justify-content: space-around;
            align-items: center;
            padding: 1rem;
            width: 100%;
            
            & > div {
                max-width: 45%;
            }

            & > div.copyright {
                display: flex;
                flex-direction: column;
                gap: 0.5rem;
            }

            & > div > p {
                text-align: justify;
                font-size: 0.6em;
            }

            & > div > p > span.material-symbols-outlined {
                font-size: 0.8rem;
            }

            & > div > hr {
                background-color: var(--color-fg);
                color: var(--color-fg);
            }
        "#
    )).expect("Failed to create style.");
    html! {
        <footer class={style}>
            <div class="logo">
                <LogoFull />
            </div>
            <div class="copyright">
                <p><MaterialIcon code={"copyright"}/> {" Copyright 2024"}</p>
                <p>{ "All Rights Reserved. Aivan Ross Anuyo aish3n@pm.me anuyoaa1@students.nu-dasma.edu.ph" }</p>
                <hr />
                <p>{ "NU School Clinic Management System is a platform for managing student and staff health records, appointments, and clinic inventory, ensuring efficient healthcare delivery and streamlined operations within the school." }</p>
            </div>
        </footer>
    }
}
