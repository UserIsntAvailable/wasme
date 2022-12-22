mod button;
mod header;

use yew::prelude::*;

use crate::components::*;
use button::SessionButton;
use chrono::{DateTime, Days, Utc};
use header::SessionHeader;

#[function_component]
fn CurrentSession() -> Html {
    // TODO: These fields needs to be inside `use_state` and then have an EventHandler `set` them
    // whenever a browser tab or window is created or closed.
    let title = "2 Windows".to_owned();
    let date = Utc::now();
    let tabs = 4;

    html! {
      <section id="current-session" class="margin-bottom-600">
        <SessionHeader text="Current Session" />
        <SessionButton
          title={title}
          date={date}
          tabs={tabs}
          is_active={true}
        />
      </section>
    }
}

#[function_component]
fn SavedSessions() -> Html {
    // Only used for testing
    fn days_ago(days: u64) -> DateTime<Utc> {
        Utc::now()
            .checked_sub_days(Days::new(days))
            .expect("date out of range from today.")
    }

    // TODO: Load sessions from local storage.

    html! {
      <section id="saved-sessions">
        <SessionHeader text="Saved Sessions">
          <Toggle
            ontoggle={
              // TODO: Pop up saved sessions sort menu
              Callback::default()
            }
            purpose={"Sort saved sessions by:"}
          >
            <img src="icons/sort.png" alt="" />
          </Toggle>
        </SessionHeader>
        <ul id="saved-sessions-list" role="list">
          <li id="saved-sessions-list-item">
            <SessionButton
              title={"New Puppy!".to_owned()}
              date={days_ago(6)}
              tabs={8}
              label={"Important".to_owned()}
            />
          </li>
          <li id="saved-sessions-list-item">
            <SessionButton
              title={"Books".to_owned()}
              date={days_ago(8)}
              tabs={19}
              label={"Hobbie".to_owned()}
            />
          </li>
          <li id="saved-sessions-list-item">
            <SessionButton
              title={"Funny".to_owned()}
              date={days_ago(8)}
              tabs={7}
              label={"Hobbie".to_owned()}
            />
          </li>
        </ul>
      </section>
    }
}

#[function_component]
pub fn Sessions() -> Html {
    html! {
      <div id="sessions-container">
        <CurrentSession />
        <SavedSessions />
      </div>
    }
}

#[function_component]
pub fn SelSession() -> Html {
    // TODO: I'm not really sure how to place heading (h*) tags here.
    // TODO: Include page icon for tab items.
    // TODO: Tab items should be <a>.
    // TODO: Toggle to expand windows tabs.
    // TODO: Find better design for `session__items > *`.

    html! {
      <section id="selected-session">
        <div class="flex margin-bottom-300">
          <header><div class="fs-pri-heading fw-bolder">{"New Puppy!"}</div></header>
          <EvenColumns>
            <Button
              onclick={
                // TODO: Open selected session.
                Callback::default()
              }
              purpose={"Opens this session's windows"}
            >
              <img src="icons/share.png" alt="" />
            </Button>
            <Button
              onclick={
                // TODO: Download selected session.
                Callback::default()
              }
              purpose={"Downloads a json representation of this section"}
            >
              <img src="icons/download.png" alt="" />
            </Button>
            <Button
              onclick={
                // TODO: Edit selected session.
                Callback::default()
              }
              purpose={"Enter edit mode to modify values from this section"}
            >
              <img src="icons/edit.png" alt="" />
            </Button>
          </EvenColumns>
        </div>
        <ul class="selected-session__metadata margin-bottom-600" role="list">
          <li>
            <span
              >{"Saved "}<time datetime="2017-06-12 13:21"
                >{"06/12/2017 1:21 PM"}</time
              ></span
            >
          </li>
          <li>{"Important"}</li>
          <li>{"2 Windows"}</li>
          <li>{"7 Tabs"}</li>
        </ul>
        <div class="margin-bottom-400">
          <ul class="selected-session__windows" role="list">
            <li class="margin-bottom-600">
              <div class="fs-ter-heading margin-bottom-200 text-dec-underline">{"Training"}</div>
              <ul class="selected-session__windows__tabs" role="list">
                <li>{"How to Train a Puppy"}</li>
                <li>{"Starting your puppy off right"}</li>
                <li>{"Dog Training Basics"}</li>
              </ul>
            </li>
            <li class="margin-bottom-600">
              <div class="fs-ter-heading margin-bottom-200 text-dec-underline">{"Food"}</div>
              <ul class="selected-session__windows__tabs" role="list">
                <li>{"Healthy Development Puppy Food"}</li>
                <li>{"Pet Food & Threats"}</li>
                <li>{"Chicken Fillets Dog Treats"}</li>
                <li>{"Puppy Bites Lamb & Salmon Threats"}</li>
              </ul>
            </li>
          </ul>
        </div>
      </section>
    }
}
