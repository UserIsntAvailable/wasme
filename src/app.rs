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
    // TODO: Load sessions from localstorage
    // TODO: How to place heading (h*) tags
    // TODO: Include images for tab items
    // TODO: tab items should be <a>

    // FIX: Add keys to list items: https://yew.rs/docs/concepts/html/lists#keyed-lists
    // FIX: I'm not really sure if my usages of <section> are correct.

    let saved_sessions = use_state(|| Html::default());
    {
        let saved_sessions = saved_sessions.clone();

        use_effect_with_deps(
            move |_| {
                let saved_sessions = saved_sessions.clone();

                saved_sessions.set([
                    Session::new("New Puppy!", days_ago(6), 8, "important"),
                    Session::new("Books", days_ago(8), 19, "hobbie"),
                    Session::new("Funny", days_ago(8), 7, "hobbie"),
                ]
                .into_iter()
                .enumerate()
                .map(|(i, s)| {
                    html! {
                      <li class="session-list-item" selected={if i == 0 { true } else { false }}>
                        <button class="box button">
                          <div class="fs-session-box-name">{s.name}</div>
                          <ul class="session-box-info-list" role="list">
                            <li>
                              <span>
                                {"Saved "}
                                <time datetime={s.date.to_string()}>
                                  {format!("{} days ago", (Utc::now().date_naive() - s.date).num_days())}
                                </time>
                              </span>
                            </li>
                            <li>{s.label}</li>
                            <li>{format!("{} Tabs", s.tabs_count)}</li>
                          </ul>
                        </button>
                      </li>
                    }
                })
                .collect());

                || ()
            },
            (),
        );
    }

    html! {
        <>
          <header class="primary-header margin-bottom-300" style="background-color: pink;">
            <div class="container">
              <div class="flex">
                <a class="header-logo" href="https://github.com/UserIsntAvailable/wasme"
                  >{"WASME"}</a
                >
                <form action="">
                  <input
                    class="header-search-bar"
                    placeholder="Search the name of a label, session, window or tab"
                    type="search"
                  />
                </form>
                <ul class="header-button-list" role="list">
                  <li>
                    <a class="button" href="https://github.com/UserIsntAvailable">
                      <img src="icons/github-mark.svg" alt="Creator's github page" />
                    </a>
                  </li>
                  <li>
                    <button class="button">
                      <img
                        src="icons/sun.png"
                        alt="Switch between dark and light mode"
                      />
                    </button>
                  </li>
                  <li>
                    <button class="button">
                      <img src="icons/nut.png" alt="Settings" />
                    </button>
                  </li>
                </ul>
              </div>
            </div>
          </header>

          <main>
            <div class="container">
              <div class="main-split">
                <div>
                  <section class="margin-bottom-600">
                    <header class="margin-bottom-200">
                      <div class="fs-sec-heading">{"Current Session"}</div>
                    </header>
                    <button class="box button">
                      <div class="fs-session-box-name">{"Changed a few seconds ago"}</div>
                      <ul class="session-box-info-list" role="list">
                        <li>{"4 Tabs"}</li>
                      </ul>
                    </button>
                  </section>
                  <section class="saved-sessions">
                    <header class="flex margin-bottom-200">
                      <div class="fs-sec-heading">{"Saved Sessions"}</div>
                      <button class="button"><img src="icons/sort.png" alt=""/></button>
                    </header>
                    <ul class="session-list" role="list">
                      { (*saved_sessions).clone() }
                    </ul>
                  </section>
                </div>
                <section class="selected-session">
                  <div class="margin-bottom-400">
                    <header><div class="session__name fs-sec-heading">{"New Puppy!"}</div></header>
                    <div class="top-info-buttons">
                      <button class="button"><img src="icons/share.png" /></button>
                      <button class="button"><img src="icons/download.png" /></button>
                      <button class="button"><img src="icons/edit.png" /></button>
                    </div>
                  </div>
                  <ul class="margin-bottom-400" role="list">
                    <li class="session__date">
                      <span>
                        {"Saved "}
                        <time datetime="2017-06-12 13:21">{"06/12/2017 1:21 PM"}</time>
                      </span>
                    </li>
                    <li>{"important"}</li>
                    <li>{"2 Windows"}</li>
                    <li>{"7 Tabs"}</li>
                  </ul>
                  <div class="bottom-info">
                    <ul class="session__windows" role="list">
                      <li class="margin-bottom-600">
                        <div class="fs-ter-heading td-underline margin-bottom-200">{"Training"}</div>
                        <ul class="session__windows__tabs" role="list">
                          <li>{"How to Train a Puppy"}</li>
                          <li>{"Starting your puppy off right"}</li>
                          <li>{"Dog Training Basics"}</li>
                        </ul>
                      </li>
                      <li class="margin-bottom-600">
                        <div class="fs-ter-heading td-underline margin-bottom-200">{"Food"}</div>
                        <ul class="session__windows__tabs" role="list">
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
