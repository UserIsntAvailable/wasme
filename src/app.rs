use chrono::{offset::Utc, Days, NaiveDate};
use yew::prelude::*;

struct Session {
    pub name: String,
    pub date: NaiveDate,
    pub tabs_count: u64,
    // TODO: Create `Label` struct to save also the color associated with it.
    pub label: String,
}

impl Session {
    fn new(
        name: impl Into<String>,
        date: NaiveDate,
        tabs_count: u64,
        label: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            date,
            tabs_count,
            label: label.into(),
        }
    }
}

fn days_ago(days: u64) -> NaiveDate {
    Utc::now()
        .date_naive()
        .checked_sub_days(Days::new(days))
        .expect("date out of range from today.")
}

#[function_component]
pub fn App() -> Html {
    // TODO: Find better Font
    // TODO: SVG logo for page
    // TODO: Help button to the right side of the search explaining how search works
    // TODO: logo and github logo should not be selectable
    // TODO: Find logo for button.saved-sessions-order-button

    // FIX: I'm not really sure if my usages of <section> are correct.
    // FIX: Calculate once and load from file.
    let saved_sessions = [
        Session::new("New Puppy!", days_ago(6), 8, "important"),
        Session::new("Books", days_ago(8), 19, "hobbie"),
        Session::new("Funny", days_ago(8), 7, "hobbie"),
    ]
    .into_iter()
    .enumerate()
    .map(|(i, s)| {
        html! {
          <li class="session-list-item" selected={if i == 0 { true } else { false }}>
            <button class="session-box">
              <h3>{s.name}</h3>
              <ul class="session-box-info-list" role="list">
                <li>
                  <span>{"Saved "}</span>
                  <time datetime={s.date.to_string()}>
                    {format!("{} days ago", (Utc::now().date_naive() - s.date).num_days())}
                  </time>
                </li>
                <li>{s.label}</li>
                <li>{format!("{} Tabs", s.tabs_count)}</li>
              </ul>
            </button>
          </li>
        }
    });

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
              <div class="main-split">
                <section class="sessions-info">
                  <section class="current-session">
                    <header><h2>{"Current Session"}</h2></header>
                    <button class="session-box">
                      <h3>{"Changed a few seconds ago"}</h3>
                      <ul class="session-box-info-list" role="list">
                        <li>{"4 Tabs"}</li>
                      </ul>
                    </button>
                  </section>
                  <section class="saved-sessions">
                    <header>
                      <h2>{"Saved Sessions"}</h2>
                      <button class="saved-sessions-order-button">{"Order by"}</button>
                    </header>
                    <ul class="session-list" role="list">
                      { for saved_sessions }
                    </ul>
                  </section>
                </section>
                <section class="selected-session-info">
                  <div class="top-info">
                    <header><h1 class="session-title">{"New Puppy!"}</h1></header>
                    <div class="top-info-buttons">
                      <button>{"OPEN"}</button>
                      <button>{"..."}</button>
                    </div>
                  </div>
                  <ul class="middle-info" role="list">
                    <li><time datetime="2017-06-12 13:21">{"Saved 06/12/2017 1:21 PM"}</time></li>
                    <li>{"important"}</li>
                    <li>{"2 Windows"}</li>
                    <li>{"7 Tabs"}</li>
                  </ul>
                  <div class="bottom-info">
                    <ul class="session-windows-list" role="list">
                      <li class="session-windows-list-item">
                        <h2>{"Training"}</h2>
                        <ul class="session-windows-tabs-list" role="list">
                          <li>{"How to Train a Puppy"}</li>
                          <li>{"Starting your puppy off right"}</li>
                          <li>{"Dog Training Basics"}</li>
                        </ul>
                      </li>
                      <li class="session-windows-list-item">
                        <h2>{"Food"}</h2>
                        <ul class="session-windows-tabs-list" role="list">
                          <li>{"Healthy Development Puppy Food"}</li>
                          <li>{"Pet Food & Threats"}</li>
                          <li>{"Chicken Fillets Dog Treats"}</li>
                          <li>{"Puppy Bites Lamb & Salmon Threats"}</li>
                        </ul>
                      </li>
                    </ul>
                  </div>
                </section>
              </div>
            </div>
          </main>
        </>
    }
}
