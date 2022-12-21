use yew::prelude::*;

#[function_component]
pub fn SearchBar() -> Html {
    // TODO: Search button
    // TODO: Search prompt
    // TODO: Section explaining search on search prompt

    let on_submit = Callback::from(|e: SubmitEvent| e.prevent_default());

    html! {
      <form onsubmit={on_submit} action="">
        <input
          class="header-search-bar"
          placeholder="Search the name of a label, session, window or tab"
          type="search"
        />
      </form>
    }
}
