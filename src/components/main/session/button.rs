use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use timeago::Formatter;
use yew::prelude::*;

static DATE_FORMATTER: Lazy<Formatter> = Lazy::new(|| Formatter::new());

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub date: DateTime<Utc>,
    pub tabs: u64,
    // TODO: Create `Label` struct to save also a color associated with it.
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub is_active: bool,
}

#[function_component]
pub fn SessionButton(
    Props {
        title,
        date,
        tabs,
        label,
        is_active,
    }: &Props,
) -> Html {
    let is_active = use_state(|| *is_active);

    let onclick = {
        let is_active = is_active.clone();
        let current = *is_active;

        if !current {
            // TODO: Reflect changes on #selected-session.
            // TODO: Set `is_active` to `false` from prev `SessionButton`.
            Callback::from(move |_| is_active.set(true))
        } else {
            Callback::default()
        }
    };

    // TODO: When date is hovered show actual date.
    // FIX: Long `session__title` and `session__label` text breaks the grid layout.

    html! {
      <button
        class="box button"
        onclick={onclick}
        aria-current={is_active.to_string()}
      >
        <div class="box__title">{title}</div>
        <ul class="box__items" role="list">
          <li class="box__items__date">
            <span>
              {"Saved "}
              <time datetime={date.to_string()}>
                {DATE_FORMATTER.convert_chrono(*date, Utc::now())}
              </time>
            </span>
          </li>
          <li class="box__items__label">{label}</li>
          <li class="box__items__tabs">{format!("{} Tabs", tabs)}</li>
        </ul>
      </button>
    }
}
