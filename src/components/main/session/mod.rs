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
          // FIX: This should use `aria-expanded`.
          <Toggle
            ontoggle={
              // TODO: Toggle saved sessions sort menu
              Callback::default()
            }
            purpose={"Sort saved sessions by:"}
          >
            <Svg viewBox="0 0 200 200">
              <Path d="M110.22,117.75h-80a10,10,0,0,0,0,20h80a10,10,0,0,0,0-20Z" />
              <Path
                d="M177.22,125.75a9.67,9.67,0,0,0-14,0l-8,7.5V42.75a10,10,0,0,0-20,0v113.5a8.29,8.29,0,0,0,3,8,9.67,9.67,0,0,0,14,0l24.5-24.5a10.13,10.13,0,0,0,.5-14Z"
              />
              <Path d="M110.22,37.75h-80a10,10,0,0,0,0,20h80a10,10,0,0,0,0-20Z" />
              <Path d="M30.22,97.75h70a10,10,0,0,0,0-20h-70a10,10,0,0,0,0,20Z" />
            </Svg>
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
              purpose={"Opens all selected session's windows"}
            >
              <Svg>
                <Path
                  d="M8.416,3.943l1.12-1.12v9.031c0,0.257,0.208,0.464,0.464,0.464c0.256,0,0.464-0.207,0.464-0.464V2.823l1.12,1.12c0.182,0.182,0.476,0.182,0.656,0c0.182-0.181,0.182-0.475,0-0.656l-1.744-1.745c-0.018-0.081-0.048-0.16-0.112-0.224C10.279,1.214,10.137,1.177,10,1.194c-0.137-0.017-0.279,0.02-0.384,0.125C9.551,1.384,9.518,1.465,9.499,1.548L7.76,3.288c-0.182,0.181-0.182,0.475,0,0.656C7.941,4.125,8.234,4.125,8.416,3.943z
                  M15.569,6.286h-2.32v0.928h2.32c0.512,0,0.928,0.416,0.928,0.928v8.817c0,0.513-0.416,0.929-0.928,0.929H4.432c-0.513,0-0.928-0.416-0.928-0.929V8.142c0-0.513,0.416-0.928,0.928-0.928h2.32V6.286h-2.32c-1.025,0-1.856,0.831-1.856,1.856v8.817c0,1.025,0.832,1.856,1.856,1.856h11.138c1.024,0,1.855-0.831,1.855-1.856V8.142C17.425,7.117,16.594,6.286,15.569,6.286z"
                />
              </Svg>
            </Button>
            // FIX: This should use `aria-expanded`.
            <Toggle
              ontoggle={
                // TODO: Toggle selected section extra menu.
                Callback::default()
              }
              purpose={"Extra menu to edit, download or delele the selected session"}
            >
              <Svg>
                <Path
                  d="M3.936,7.979c-1.116,0-2.021,0.905-2.021,2.021s0.905,2.021,2.021,2.021S5.957,11.116,5.957,10
                  S5.052,7.979,3.936,7.979z
                  M3.936,11.011c-0.558,0-1.011-0.452-1.011-1.011s0.453-1.011,1.011-1.011S4.946,9.441,4.946,10
                  S4.494,11.011,3.936,11.011z
                  M16.064,7.979c-1.116,0-2.021,0.905-2.021,2.021s0.905,2.021,2.021,2.021s2.021-0.905,2.021-2.021
                  S17.181,7.979,16.064,7.979z
                  M16.064,11.011c-0.559,0-1.011-0.452-1.011-1.011s0.452-1.011,1.011-1.011S17.075,9.441,17.075,10
                  S16.623,11.011,16.064,11.011z
                  M10,7.979c-1.116,0-2.021,0.905-2.021,2.021S8.884,12.021,10,12.021s2.021-0.905,2.021-2.021
                  S11.116,7.979,10,7.979z
                  M10,11.011c-0.558,0-1.011-0.452-1.011-1.011S9.442,8.989,10,8.989S11.011,9.441,11.011,10
                  S10.558,11.011,10,11.011z"
                />
              </Svg>
            </Toggle>
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
