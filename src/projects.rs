use yew::prelude::*;
use yew_router::components::Link;

use crate::router::Route;

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <div class="projects">
            <div class="project">
                <h3>{"This Website!"}</h3>
            </div>

            <div class="project">
                <h3>{"Video Games"}</h3>
                <p>{"If the "}<Link<Route> to={Route::Games}>{"tab"}</Link<Route>>{" on the left didn't give it away already, I make games! None in recent times I must admit (although that could be subject to change). In the mean time, do feel free to check out the previous games I have made!"}</p>

                <img src="static/images/Memories.png" />
            </div>
        </div>
    }
}
