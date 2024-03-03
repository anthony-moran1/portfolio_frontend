use crate::sidebar::Sidebar;
use yew::prelude::*;

#[function_component(LeftContent)]
pub fn left_content() -> Html {
    html! {
    <div style="position: sticky; top: 0; z-index: 1;">
    <div id="left-content-parent" class="flex vh-100">
        <div id="left-content" class="flex-column">
            <header><h1><a href="/">{"Anthony Moran"}</a></h1></header>
            <Sidebar />
        </div>

        <div id="left-content-tab-parent" class="vh-100">
            <div id="left-content-tab"></div>
        </div>
    </div>
    </div>
    }
}
