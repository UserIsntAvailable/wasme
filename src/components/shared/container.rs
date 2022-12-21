use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component]
pub fn Container(Props { children }: &Props) -> Html {
    html! {
      <div class="container">
        { for children.iter() }
      </div>
    }
}
