use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component]
pub fn EvenColumns(Props { children }: &Props) -> Html {
    html! {
      <div id="button-group" class="even-columns">
        { for children.iter() }
      </div>
    }
}
