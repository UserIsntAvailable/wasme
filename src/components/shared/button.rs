use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    pub onclick: Callback<MouseEvent>,
    pub purpose: &'static str,
}

#[function_component]
pub fn Button(
    Props {
        children,
        onclick,
        purpose,
    }: &Props,
) -> Html {
    html! {
      <button class="button" onclick={onclick}>
        { for children.iter() }
        <span class="visually-hidden">{purpose}</span>
      </button>
    }
}
