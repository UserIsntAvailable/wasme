mod components;

use crate::components::{*, header::MainHeader};
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
    // TODO: Logo and github logo should not be selectable
    // TODO: Load sessions from localstorage
    // TODO: How to better place heading (h*) tags
    // TODO: Include page icon for tab items
    // TODO: Tab items should be <a>
    // TODO: Toggle to expand windows tabs
    // TODO: Find better design for `session__items > *`
    // TODO: Responsive Layout
    // TODO: Skip link

    // FIX: Accessibility
    // FIX: Long `session__title` and `session__label` names break the grid layout.
    // FIX: Images get smaller with small screen sizes ( I think that is the normal behaviour of
    // flex ).
    // FIX: Add keys to list items: https://yew.rs/docs/concepts/html/lists#keyed-lists
    // FIX: I'm not really sure if my usages of <section> are correct.

    let saved_sessions = use_state(|| Html::default());
    {
        let saved_sessions = saved_sessions.clone();

        use_effect_with_deps(
            move |_| {
                let saved_sessions = saved_sessions.clone();

                saved_sessions.set([
                    Session::new("New Puppy!", days_ago(6), 8, "Important"),
                    Session::new("Books", days_ago(8), 19, "Hobbie"),
                    Session::new("Funny", days_ago(8), 7, "Hobbie"),
                ]
                .into_iter()
                .enumerate()
                .map(|(i, s)| {
                    html! {
                      <li class="session-list-item" selected={if i == 0 { true } else { false }}>
                        <button class="box button">
                          <div class="box__title">{s.name}</div>
                          <ul class="box__items" role="list">
                            <li class="box__items__date">
                              <span>
                                {"Saved "}
                                <time datetime={s.date.to_string()}>
                                  {format!("{} days ago", (Utc::now().date_naive() - s.date).num_days())}
                                </time>
                              </span>
                            </li>
                            <li class="box__items__label">{s.label}</li>
                            <li class="box__items__tabs">{format!("{} Tabs", s.tabs_count)}</li>
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
          <MainHeader />

          <main>
            <Container>
              <div class="main-split">
                <div>
                  <section class="margin-bottom-600">
                    <header class="margin-bottom-200">
                      <h2 class="fs-sec-heading">{"Current Session"}</h2>
                    </header>
                    <button class="box button">
                      <div class="box__title">{"Changed a few seconds ago"}</div>
                      <ul class="box__items" role="list">
                        <li class="box__items__tabs" style="grid-area: date; text-align: left;">{"4 Tabs"}</li>
                      </ul>
                    </button>
                  </section>
                  <section class="saved-sessions">
                    <header class="flex margin-bottom-200">
                      <h2 class="fs-sec-heading">{"Saved Sessions"}</h2>
                      <button class="button"><img src="icons/sort.png" alt=""/></button>
                    </header>
                    <ul class="session-list" role="list">
                      { (*saved_sessions).clone() }
                    </ul>
                  </section>
                </div>
                <section class="selected-session">
                  <div class="flex margin-bottom-300">
                    <header><div class="fs-pri-heading fw-bolder">{"New Puppy!"}</div></header>
                    <div class="flex">
                      <button class="button"><img src="icons/share.png" /></button>
                      <button class="button"><img src="icons/download.png" /></button>
                      <button class="button"><img src="icons/edit.png" /></button>
                    </div>
                  </div>
                  <ul class="session__items margin-bottom-600" role="list">
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
                  <div class="margin-bottom-400">
                    <ul class="session__windows" role="list">
                      <li class="margin-bottom-600">
                        <div class="fs-ter-heading margin-bottom-200 text-dec-underline">{"Training"}</div>
                        <ul class="session__windows__tabs" role="list">
                          <li>{"How to Train a Puppy"}</li>
                          <li>{"Starting your puppy off right"}</li>
                          <li>{"Dog Training Basics"}</li>
                        </ul>
                      </li>
                      <li class="margin-bottom-600">
                        <div class="fs-ter-heading margin-bottom-200 text-dec-underline">{"Food"}</div>
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
            </Container>
          </main>
        </>
    }
}
