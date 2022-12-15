use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    // TODO: Logo for page.
    // TODO: Maybe Search bar + filters in the middle of the `nav-flex`?
    html! {
        <>
          <header class="primary-header">
            <div class="container">
              <div class="nav-flex">
                <a class="nav-logo" href="https://github.com/UserIsntAvailable/wasme">{"WASME"}</a>
                <nav>
                  <ul class="nav-button-list" role="list">
                    <li>
                      <a class="nav-button" href="https://github.com/UserIsntAvailable">
                        <img src="icons/github-mark.svg" alt="Creator's github page" />
                      </a>
                    </li>
                    <li>
                      <button class="nav-button">
                        <img
                          src="icons/sun.png"
                          alt="Switch between dark and light mode"
                        />
                      </button>
                    </li>
                    <li>
                      <button class="nav-button"><img src="icons/nut.png" alt="Settings" /></button>
                    </li>
                  </ul>
                </nav>
              </div>
            </div>
          </header>

          <main>
            <section class="">
            </section>
          </main>
        </>
    }
}
