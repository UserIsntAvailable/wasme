use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    // TODO: Add aria-controls prop later.
    pub children: Children,
    pub id: Option<&'static str>,
    pub default: Option<bool>,
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
    let toggle = use_state(|| default.unwrap_or(false));

    let onclick = {
        let toggle = toggle.clone();
        let ontoggle = ontoggle.clone();

        Callback::from(move |_| {
            let current = *toggle;

            ontoggle.emit(current);
            toggle.set(!current);
        })
    };

    html! {
      <button id={id.clone()} class="button" onclick={onclick} aria-expanded={toggle.to_string()}>
        { for children.iter() }
        <span class="visually-hidden">{purpose}</span>
      </button>
    }
}
