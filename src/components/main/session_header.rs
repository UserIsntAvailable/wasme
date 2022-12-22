use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text: &'static str,
    /// Optional
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn SessionHeader(Props { text, children }: &Props) -> Html {
    html! {
      <header class="flex margin-bottom-200">
        <h2 class="fs-sec-heading">{text}</h2>
        { for children.iter() }
      </header>
    }
}
