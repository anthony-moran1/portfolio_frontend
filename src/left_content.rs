use crate::sidebar::Sidebar;
use yew::prelude::*;

#[function_component(LeftContent)]
pub fn left_content() -> Html {
    html! {
        <div id="left-content" class="flex-column">
            <header><h1><a href="/">{"Anthony Moran"}</a></h1></header>
            <Sidebar />
        </div>
    }
}
