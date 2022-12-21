use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component]
pub fn ButtonGroup(Props { children }: &Props) -> Html {
    html! {
      <div id="button-group" class="flex">
        { for children.iter() }
      </div>
    }
}
