use stylist::{css, Style};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let onclick = props.onclick.clone().unwrap_or_else(|| Callback::from(|_| {}));

    let style = Style::new(css!(
        r#"
            appearance: button;
            background-color: var(--color-blue-dark);
            border: solid transparent;
            border-radius: 0.2rem;
            border-width: 0 0 0.2rem;
            color: var(--color-fg);
            display: inline-block;
            font-weight: 700;
            outline: none;
            padding: 0.5rem 1rem 0.2rem 1rem;
            text-align: center;
            touch-action: manipulation;
            transform: translateZ(0);
            transition: filter .2s;
            user-select: none;
            -webkit-user-select: none;
            vertical-align: middle;
            white-space: nowrap;
            filter: brightness(1.1);
            -webkit-filter: brightness(1.1);

            &:after {
                background-clip: padding-box;
                background-color: var(--color-blue);
                border: solid transparent;
                border-radius: 0.2rem;
                border-width: 0 0 4px;
                bottom: -4px;
                content: "";
                left: 0;
                position: absolute;
                right: 0;
                top: 0;
                z-index: -1;
            }

            &,
            &:focus {
                user-select: auto;
            }

            &:hover:not(:disabled) {
                filter: brightness(1.3);
                -webkit-filter: brightness(1.3);
            }

            &:disabled {
                cursor: auto;
            }

            &:active {
                border-width: 4px 0 0;
                background: none;
            }
        "#
    ))
    .expect("Failed to create style.");

    html! {
        <button class={classes!(style, props.class.clone())} {onclick}>
            { for props.children.iter() }
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct RedirectButtonProps<T: Routable + Clone> {
    pub route: T,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(RedirectButton)]
pub fn redirect_button<T: Routable + Clone + 'static>(props: &RedirectButtonProps<T>) -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = {
        let route = props.route.clone();
        Callback::from(move |_| navigator.push(&route))
    };

    html! {
        <Button class={props.class.clone()} {onclick}>{ for props.children.iter() }</Button>
    }
}
