mod search_bar;

use crate::components::*;
use search_bar::SearchBar;
use yew::prelude::*;

#[function_component]
pub fn MainHeader() -> Html {
    // TODO: svg logo for page.
    // TODO: header-logo and github-logo should not be selectable.

    html! {
      <header class={classes!("main-header", "margin-bottom-300")}>
        <Container>
          <div id="header-contents" class="flex-centered">
            <Link class="header-logo" href="https://github.com/UserIsntAvailable/wasme"
              >{"WASME"}</Link
            >
            <SearchBar />
            <EvenColumns>
              <Link class="button" href="https://github.com/UserIsntAvailable">
                <Img src="icons/github-mark.svg" alt="(opens in new tab)" />
                <span class="visually-hidden">{"Creator's Github Page"}</span>
              </Link>
              <Toggle
                ontoggle={
                  // TODO: Swap icon and theme
                  Callback::default()
                }
                purpose={"Change between light and dark theme"}
              >
                <Img src="icons/sun.png" />
              </Toggle>
              <Toggle
                ontoggle={
                  // TODO: Pop up settings menu
                  Callback::default()
                }
                purpose={"Opens settings menu"}
              >
                <Img src="icons/nut.png" />
              </Toggle>
            </EvenColumns>
          </div>
        </Container>
      </header>
    }
}
