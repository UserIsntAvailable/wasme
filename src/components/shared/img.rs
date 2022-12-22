use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub src: &'static str,
    #[prop_or_default]
    pub alt: &'static str,
}

#[function_component]
pub fn Img(Props { src, alt }: &Props) -> Html {
    html! {
      <img src={*src} alt={*alt}/>
    }
}
