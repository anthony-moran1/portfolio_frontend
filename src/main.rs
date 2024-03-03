mod layout;
mod router;

mod not_implemented;

mod about_me;
mod index;
mod projects;

mod settings;
mod socials;

mod blog;
mod games;
mod left_content;
mod sidebar;
mod sidebar_item;

use yew::prelude::*;
use yew_router::prelude::*;

use router::{switch, Route};

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
