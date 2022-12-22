mod components;

use crate::components::{header::MainHeader, main::Main};
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    // TODO: Find better Font
    // TODO: Skip link
    // TODO: Responsive Layout
    // TODO: Config to select where #section-session is placed ( left or right )

    // FIX: Accessibility
    // FIX: Images get smaller with small screen sizes ( I think that is the normal behaviour of
    // flex ).
    // FIX: Add keys to list items: https://yew.rs/docs/concepts/html/lists#keyed-lists
    // FIX: I'm not really sure if my usages of <section> are correct.

    html! {
        <>
          <MainHeader />
          <Main />
        </>
    }
}
