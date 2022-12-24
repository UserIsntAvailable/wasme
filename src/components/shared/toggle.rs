use yew::prelude::*;
use yew_hooks::use_bool_toggle;

#[derive(Properties, PartialEq)]
pub struct Props {
    // TODO: Add aria-controls prop later.
    pub children: Children,
    pub id: Option<&'static str>,
    #[prop_or_default]
    pub default: bool,
    // FIX: Maybe this should directly take an `UseToggleHandle`.
    //
    // The only thing that can block that, is that `UseToggleHandle` doesn't implement `Default`, so
    // every another component is forced to express their state using `UseToggleHandle`. Whenever I
    // implement other `Toggle` `ontoggle` callbacks, it will be more clear if this makes sense.
    pub ontoggle: Callback<bool>,
    pub purpose: String,
}

#[function_component]
pub fn Toggle(
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
      <button id={id.clone()} class="button" onclick={onclick} aria-pressed={toggle.to_string()}>
        { for children.iter() }
        <span class="visually-hidden">{purpose}</span>
      </button>
    }
}
