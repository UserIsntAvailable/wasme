use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    // TODO: Logo for page.
    // TODO: Help button to the right side of the search
    // TODO: logo and github logo should not be selectable
    html! {
        <>
          <header class="primary-header">
            <div class="container">
              <div class="nav-flex">
                <a class="nav-logo" href="https://github.com/UserIsntAvailable/wasme"
                  >{"WASME"}</a
                >
                <form action="">
                  <input
                    class="header-search-bar"
                    placeholder="Search the name of a label, session, window or tab"
                    type="search"
                  />
                </form>
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
                      <button class="nav-button">
                        <img src="icons/nut.png" alt="Settings" />
                      </button>
                    </li>
                  </ul>
                </nav>
              </div>
            </div>
          </header>

          <main>
            <div class="container">
              <div class="main-grid">
                <section class="main-sessions">{"Hello"}</section>
                <section class="main-tabs">{"World"}</section>
              </div>
            </div>
          </main>
        </>
    }
}
