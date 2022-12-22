use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    pub class: &'static str,
    pub href: &'static str,
    #[prop_or("_blank")]
    pub target: &'static str,
}

#[function_component]
pub fn Link(Props { children, class, href, target }: &Props) -> Html {
    html! {
      <a class={*class} href={*href} target={*target}>
        { for children.iter() }
      </a>
    }
}
