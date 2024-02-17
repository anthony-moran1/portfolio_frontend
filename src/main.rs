mod layout;
mod router;

mod about_me;
mod index;
mod projects;

mod settings;
mod socials;

mod left_content;
mod sidebar;
mod sidebar_item;
mod blog;
mod games;

use yew::prelude::*;
use yew_router::prelude::*;

use router::{switch, Route};

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
