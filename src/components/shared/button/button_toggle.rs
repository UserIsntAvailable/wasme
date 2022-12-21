use yew::prelude::*;
use yew_hooks::use_bool_toggle;

#[derive(Properties, PartialEq)]
pub struct Props {
    // TODO: Add aria-controls prop later.
    pub children: Children,
    pub id: Option<&'static str>,
    #[prop_or_default]
    pub default: bool,
    pub ontoggle: Callback<bool>,
    pub purpose: &'static str,
}

#[function_component]
pub fn ButtonToggle(
    Props {
        id,
        children,
        default,
        ontoggle,
        purpose,
    }: &Props,
) -> Html {
    let toggle = use_bool_toggle(*default);

    let onclick = {
        let toggle = toggle.clone();
        let ontoggle = ontoggle.clone();

        Callback::from(move |_| {
            toggle.toggle();
            ontoggle.emit(*toggle);
        })
    };

    html! {
      <button id={id.clone()} class="button" onclick={onclick} aria-expanded={toggle.to_string()}>
        { for children.iter() }
        <span class="visually-hidden">{purpose}</span>
      </button>
    }
}
