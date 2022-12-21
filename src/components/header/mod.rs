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
            <a class="header-logo" target="_blank" href="https://github.com/UserIsntAvailable/wasme"
              >{"WASME"}
            </a>
            <SearchBar />
            <ButtonGroup>
              <a class="button" target="_blank" href="https://github.com/UserIsntAvailable">
                <img alt="(opens in new tab)" src="icons/github-mark.svg" />
                <span class="visually-hidden">{"Creator's Github Page"}</span>
              </a>
              <ButtonToggle
                ontoggle={
                  // TODO: Swap icon and theme
                  Callback::default()
                }
                purpose={"Change between light and dark theme"}
              >
                <img src="icons/sun.png" alt="" />
              </ButtonToggle>
              <ButtonToggle
                ontoggle={
                  // TODO: Pop up settings menu
                  Callback::default()
                }
                purpose={"Opens settings menu"}
              >
                <img src="icons/nut.png" alt="" />
              </ButtonToggle>
            </ButtonGroup>
          </div>
        </Container>
      </header>
    }
}
