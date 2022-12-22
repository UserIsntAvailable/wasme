mod session;

use crate::components::{
    main::session::{SelSession, Sessions},
    *,
};
use yew::prelude::*;

#[function_component]
pub fn Main() -> Html {
    html! {
      <main>
        <Container>
          <div class="main-split">
            <Sessions />
            <SelSession />
          </div>
        </Container>
      </main>
    }
}
