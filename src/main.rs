mod router;
mod layout;

mod index;
mod projects;
mod about_me;

mod settings;

mod sidebar;
mod sidebar_item;

use yew::prelude::*;
use yew_router::prelude::*;

use router::{Route, switch};

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
