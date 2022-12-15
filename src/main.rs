mod app;

use app::App;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
