use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PProps {
    pub d: &'static str,
}

#[function_component]
pub fn Path(PProps { d }: &PProps) -> Html {
    html! {
      <path d={*d} />
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: ChildrenWithProps<Path>,
    #[prop_or("auto")]
    pub height: &'static str,
    #[prop_or("auto")]
    pub width: &'static str,
    #[prop_or("0 0 20 20")]
    pub viewBox: &'static str,
}

#[function_component]
pub fn Svg(Props { children, height, width, viewBox }: &Props) -> Html {
    html! {
      <svg
        class="svg-icon"
        fill="var(--c-neutral-900)"
        height={*height}
        width={*width}
        viewBox={*viewBox}
      >
        { for children.iter() }
      </svg>
    }
}
